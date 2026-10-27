// Xcare: portable PC cleaner and optimizer for Windows. It only does what
// actually helps (junk files, startup programs, power plan, Windows' own
// repair tools) and leaves out registry cleaners and "RAM boosters".
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod apps;
mod cleaner;
mod force_delete;
mod portable;
mod power;
mod startup;
mod updater;
mod win;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fs, sync::Mutex};
use tauri::{AppHandle, Emitter, Manager, State};
use wmi::{COMLibrary, Variant, WMIConnection};

/// Settings kept in Data/xcare.json beside the executable.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Settings {
    language: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self { language: "en".into() }
    }
}

struct AppState {
    force_delete: force_delete::Service,
    settings: Mutex<Settings>,
    /// Files found by the last scan, per category; cleaning deletes only these.
    scanned: Mutex<HashMap<String, cleaner::Files>>,
    /// Drives Windows reports as not healthy; None until the startup check ends.
    unhealthy_disks: Mutex<Option<Vec<String>>>,
    /// A newer release found on GitHub, waiting for the user to install it.
    update: Mutex<Option<updater::Update>>,
}

fn save_settings(settings: &Settings) {
    if let Ok(text) = serde_json::to_string_pretty(settings) {
        let _ = fs::write(portable::data_dir().join("xcare.json"), text);
    }
}

fn load_settings() -> Settings {
    match fs::read_to_string(portable::data_dir().join("xcare.json")) {
        // Some editors save UTF-8 with a BOM, which serde_json rejects.
        Ok(text) => serde_json::from_str(text.trim_start_matches('\u{feff}')).unwrap_or_default(),
        Err(_) => {
            let settings = Settings::default();
            save_settings(&settings);
            settings
        }
    }
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
struct Status {
    admin: bool,
    drive: String,
    free: u64,
    total: u64,
    uptime: u64,
    memory_load: u32,
    recycle_bin: u64,
    unhealthy_disks: Vec<String>,
}

#[tauri::command]
fn get_settings(state: State<AppState>) -> Settings {
    state.settings.lock().unwrap().clone()
}

#[tauri::command]
fn set_language(state: State<AppState>, language: String) {
    let mut settings = state.settings.lock().unwrap();
    settings.language = language;
    save_settings(&settings);
}

#[tauri::command]
async fn get_status(app: AppHandle) -> Status {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        // The drive check starts with the app; wait for it (up to 5 s) so the
        // first overview does not call a failing drive healthy.
        for _ in 0..50 {
            if state.unhealthy_disks.lock().unwrap().is_some() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        let unhealthy_disks = state.unhealthy_disks.lock().unwrap().clone().unwrap_or_default();
        let drive = std::env::var("SystemDrive").unwrap_or_else(|_| "C:".into());
        let (free, total) = win::disk_space(&format!("{drive}\\")).unwrap_or((0, 0));
        Status {
            admin: win::is_admin(),
            drive,
            free,
            total,
            uptime: win::uptime_seconds(),
            memory_load: win::memory_load(),
            recycle_bin: win::recycle_bin().0,
            unhealthy_disks,
        }
    })
    .await
    .unwrap_or_default()
}

#[tauri::command]
async fn scan_junk(app: AppHandle) -> Vec<cleaner::Found> {
    tauri::async_runtime::spawn_blocking(move || {
        let (found, lists) = cleaner::scan(win::is_admin(), &mut |p| {
            let _ = app.emit("scan-progress", p);
        });
        *app.state::<AppState>().scanned.lock().unwrap() = lists;
        found
    })
    .await
    .unwrap_or_default()
}

#[tauri::command]
async fn clean_junk(app: AppHandle, ids: Vec<String>) -> cleaner::Cleaned {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        let mut scanned = state.scanned.lock().unwrap();
        let result = cleaner::clean(&ids, &scanned, win::is_admin(), &mut |p| {
            let _ = app.emit("clean-progress", p);
        });
        // The lists are spent; a new scan is needed before cleaning again.
        scanned.clear();
        result
    })
    .await
    .unwrap_or_default()
}

#[tauri::command]
fn list_startup() -> Vec<startup::Item> {
    startup::list()
}

#[tauri::command]
fn set_startup(id: String, enabled: bool) -> Result<(), String> {
    startup::set(&id, enabled)
}

#[tauri::command]
async fn list_apps() -> Vec<apps::App> {
    tauri::async_runtime::spawn_blocking(apps::list).await.unwrap_or_default()
}

#[tauri::command]
fn uninstall_app(id: String) -> Result<(), String> {
    apps::uninstall(&id)
}

#[tauri::command]
fn list_power_plans() -> Vec<power::Plan> {
    power::list()
}

#[tauri::command]
fn set_power_plan(guid: String) -> Result<(), String> {
    power::set_active(&guid)
}

#[tauri::command]
fn add_ultimate_plan() -> Result<(), String> {
    power::add_ultimate()
}

#[tauri::command]
async fn pick_force_delete(window: tauri::WebviewWindow, folder: bool) -> Result<Vec<String>, String> {
    let owner = window.hwnd().map_err(|e| e.to_string())?.0 as isize;
    tauri::async_runtime::spawn_blocking(move || force_delete::pick(folder, owner))
        .await.map_err(|e| e.to_string())?
}

#[tauri::command]
async fn prepare_force_delete(app: AppHandle, paths: Vec<String>) -> Result<force_delete::Preview, String> {
    tauri::async_runtime::spawn_blocking(move || app.state::<AppState>().force_delete.prepare(paths))
        .await.map_err(|e| e.to_string())?
}

#[tauri::command]
async fn execute_force_delete(app: AppHandle, token: String, mode: String) -> Result<force_delete::Outcome, String> {
    tauri::async_runtime::spawn_blocking(move || app.state::<AppState>().force_delete.execute(&token, &mode))
        .await.map_err(|e| e.to_string())?
}

/// Windows' own tools. Only these ids are accepted.
#[tauri::command]
fn run_tool(tool: String) -> Result<(), String> {
    let (verb, file, params) = match tool.as_str() {
        "repair" => ("runas", "cmd.exe", "/k DISM /Online /Cleanup-Image /RestoreHealth && sfc /scannow"),
        "restore" => (
            "runas",
            "powershell.exe",
            "-NoProfile -NoExit -Command \"Checkpoint-Computer -Description 'Xcare' -RestorePointType MODIFY_SETTINGS; 'Restore point created.'\"",
        ),
        "defrag" => ("open", "dfrgui.exe", ""),
        "visual" => ("open", "SystemPropertiesPerformance.exe", ""),
        "gamemode" => ("open", "ms-settings:gaming-gamemode", ""),
        "storage" => ("open", "ms-settings:storagesense", ""),
        "update" => ("open", "ms-settings:windowsupdate", ""),
        "taskmgr" => ("open", "taskmgr.exe", ""),
        "cleanmgr" => ("open", "cleanmgr.exe", ""),
        _ => return Err("unknown tool".into()),
    };
    if win::shell(verb, file, params) {
        Ok(())
    } else {
        Err("cancelled".into())
    }
}

/// Looks for a newer release; quietly None when offline.
#[tauri::command]
async fn check_update(app: AppHandle) -> Option<updater::Update> {
    tauri::async_runtime::spawn_blocking(move || {
        let update = updater::check().ok().flatten();
        *app.state::<AppState>().update.lock().unwrap() = update.clone();
        update
    })
    .await
    .ok()
    .flatten()
}

/// Downloads and verifies the update, swaps the exe and restarts.
#[tauri::command]
async fn install_update(app: AppHandle) -> Result<(), String> {
    let update = app.state::<AppState>().update.lock().unwrap().clone().ok_or("no update")?;
    tauri::async_runtime::spawn_blocking(move || updater::install(&update))
        .await
        .map_err(|e| e.to_string())??;
    app.exit(0);
    Ok(())
}

/// Opens the Skale Club website in the default browser.
#[tauri::command]
fn open_site() {
    win::shell("open", "https://skale.club", "");
}

/// Relaunches Xcare with a UAC prompt; this copy closes if the user accepts.
#[tauri::command]
fn restart_as_admin(app: AppHandle) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    if win::shell("runas", &exe.display().to_string(), "") {
        app.exit(0);
        Ok(())
    } else {
        Err("cancelled".into())
    }
}

/// Drives Windows' storage management reports as Warning (1) or Unhealthy (2).
fn unhealthy_disks() -> Vec<String> {
    let Ok(com) = COMLibrary::new() else { return Vec::new() };
    let Ok(con) = WMIConnection::with_namespace_path("ROOT\\Microsoft\\Windows\\Storage", com) else {
        return Vec::new();
    };
    let rows: Vec<HashMap<String, Variant>> =
        con.raw_query("SELECT FriendlyName, HealthStatus FROM MSFT_PhysicalDisk").unwrap_or_default();
    rows.iter()
        .filter(|r| {
            let code = match r.get("HealthStatus") {
                Some(Variant::UI1(v)) => *v as i64,
                Some(Variant::UI2(v)) => *v as i64,
                Some(Variant::UI4(v)) => *v as i64,
                Some(Variant::I2(v)) => *v as i64,
                Some(Variant::I4(v)) => *v as i64,
                _ => 0,
            };
            code == 1 || code == 2
        })
        .filter_map(|r| match r.get("FriendlyName") {
            Some(Variant::String(s)) => Some(s.clone()),
            _ => None,
        })
        .collect()
}

/// WMI needs its own COM thread, so the drive check runs once in the
/// background and always records a result, even an empty one.
fn check_disks(app: AppHandle) {
    std::thread::spawn(move || {
        let result = unhealthy_disks();
        *app.state::<AppState>().unhealthy_disks.lock().unwrap() = Some(result);
    });
}

fn main() {
    updater::finish_previous();
    portable::migrate(&["xcare.json"]);
    portable::sync_appinfo();
    tauri::Builder::default()
        .manage(AppState {
            force_delete: force_delete::Service::default(),
            settings: Mutex::new(load_settings()),
            scanned: Mutex::new(HashMap::new()),
            unhealthy_disks: Mutex::new(None),
            update: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            get_settings,
            set_language,
            get_status,
            scan_junk,
            clean_junk,
            list_startup,
            set_startup,
            list_apps,
            uninstall_app,
            list_power_plans,
            set_power_plan,
            add_ultimate_plan,
            run_tool,
            pick_force_delete,
            prepare_force_delete,
            execute_force_delete,
            restart_as_admin,
            open_site,
            check_update,
            install_update
        ])
        .setup(|app| {
            check_disks(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to start Xcare");
}
