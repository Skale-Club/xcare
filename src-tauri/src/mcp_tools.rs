//! What Xcare offers over MCP (`Xcare.exe --mcp`): the PC's state and what
//! the window would show before acting. Only reading: cleaning, startup
//! changes, uninstalling and power plans stay in the window.

use crate::mcp::{self, Tool};
use crate::{apps, cleaner, power, startup, win};
use serde_json::json;

const INSTRUCTIONS: &str = "Xcare is a PC cleaner and optimizer for Windows. status gives the system \
drive's space, uptime, memory load, Recycle Bin size and drives Windows reports as unhealthy; \
junk_files measures what each cleaning category would free (bytes, files; locked means it needs \
administrator rights); startup_items, installed_apps and power_plans list what the window manages. \
Nothing is cleaned or changed from here: tell the user what to do in Xcare.";

/// Runs WMI and COM work on a thread of its own, with its own COM setup.
fn apart<T: Send + 'static>(work: impl FnOnce() -> T + Send + 'static) -> Result<T, String> {
    std::thread::spawn(work).join().map_err(|_| "Stopped unexpectedly.".to_string())
}

pub fn serve() {
    let tools = vec![
        Tool::new(
            "status",
            "The system drive's free and total bytes, uptime in seconds, memory load in percent, \
             Recycle Bin bytes, unhealthy drives, and whether this runs as administrator.",
            mcp::no_input(),
            |_| Ok(json!(crate::read_status(apart(crate::unhealthy_disks)?))),
        ),
        Tool::new(
            "junk_files",
            "Scans the cleaning categories (temporary files, caches, logs, crash dumps, Recycle Bin...) \
             and returns, per category id, the bytes and files it would free. Deletes nothing.",
            mcp::no_input(),
            |_| Ok(json!(cleaner::scan(win::is_admin(), &mut |_| {}).0)),
        ),
        Tool::new(
            "startup_items",
            "Programs that start with Windows: name, command, where it is registered, enabled, and \
             whether changing it needs administrator rights.",
            mcp::no_input(),
            |_| Ok(json!(startup::list())),
        ),
        Tool::new(
            "installed_apps",
            "Installed programs: name, publisher, version, estimated size in bytes and install date.",
            mcp::no_input(),
            |_| Ok(json!(apps::list())),
        ),
        Tool::new(
            "power_plans",
            "Windows power plans and which one is active.",
            mcp::no_input(),
            |_| Ok(json!(power::list())),
        ),
    ];
    mcp::serve("Xcare", "xcare.json", INSTRUCTIONS, tools);
}
