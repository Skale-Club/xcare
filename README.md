# Xcare

Portable PC cleaner and optimizer for Windows, in the spirit of CCleaner and
Advanced SystemCare, but only with what actually helps. One small exe, no
installer. 🇧🇷 [Português](#português) · 🇪🇸 [Español](#español)

Made by **[Skale Club](https://skale.club)**.

## Download

Get the latest `Xcare.exe` from **[Releases](https://github.com/Skale-Club/xcare/releases/latest)**. It keeps itself up to date: when a new version is out, the app offers it (at the top of the window) and swaps itself in place, keeping your settings. Every update is signed and checked before it is used.

## Run it

Double-click `Xcare.exe`. It needs WebView2, which comes with Windows 11.
The language selector is at the bottom left (English, Português, Español) and
is saved in `Data\xcare.json` next to the exe. The folder follows the PortableApps layout: the exe, `App\AppInfo\` (name,
version, icon) and `Data\` for everything the app saves.

It runs as a normal user. Some Windows folders and programs set up for all
users need administrator rights: use **Restart as administrator** inside the
app when you want those too (Windows asks for permission).

## Pages

| Page | What it does |
|---|---|
| Overview | Checks the PC on open and gives a 0–100 score: junk files, startup programs, free space, time since restart, power plan, drive health, memory. Each problem has a button to fix it. |
| Cleaner | Scans and shows the size of each category, then deletes only what you ticked, after a confirmation. |
| Startup | Turns programs that start with Windows on and off. |
| Apps | Every installed program with size and install date; opens the program's own uninstaller. |
| Optimize | Power plan (including adding Ultimate Performance) and shortcuts to Windows' own tools: repair system files (DISM + SFC), restore point, optimize drives, visual effects, Game Mode, Storage Sense, Windows Update, Task Manager, Disk Cleanup. |

## How the cleaner stays safe

- Only caches and leftovers that Windows and apps rebuild: temp folders,
  thumbnail cache, error reports, Windows Update downloads, Delivery
  Optimization, memory dumps and browser caches (Chrome, Edge, Brave, Opera,
  Firefox). **Logins, history, cookies and passwords are never touched.**
- Temp folders keep anything from the last 24 hours, so running installers
  keep their files.
- It deletes exactly the files the scan listed, only inside each category's
  own folders, never following links or junctions. Files in use are skipped.
- Shader caches and the Recycle Bin are off by default (games rebuild shaders
  with a brief stutter; the Recycle Bin is permanent).

## Startup switch

Turning a program off writes the same "StartupApproved" flag the Task Manager
uses. The program's own entry stays, so you can turn it back on anytime, here
or in Task Manager.

## Left out on purpose

Registry cleaners, "RAM boosters" and "internet boosters". They don't make
Windows faster: registry cleaners are a common cause of broken programs, and
emptying RAM forces Windows to reload what it had cached.

## Build from source

This is a Tauri v2 app (Rust + plain HTML/JS, no bundler).
Needs Rust, Node.js and the Visual C++ Build Tools.

```bash
npm install
npm run build    # src-tauri/target/release/xcare.exe
```

```bash
cd src-tauri
cargo test --release    # cleaner safety, startup flag, powercfg parsing
```

- `src-tauri/src/cleaner.rs`: categories, scan and safe delete (with tests).
- `src-tauri/src/startup.rs`, `apps.rs`, `power.rs`: startup programs, installed apps, power plans.
- `src-tauri/src/win.rs`: Recycle Bin, ShellExecute, disk space, memory.
- `src/app.js`, `src/style.css`: the window; `src/i18n.js`: English, Portuguese, Spanish.
- `make-icon.ps1`: draws the app icon.

## Releasing a new version

```powershell
./scripts/release.ps1 0.2.0
```

It sets the version in `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json` and `package.json`, commits, tags `v0.2.0` and pushes. GitHub Actions then builds the exe, signs it with the update key (the `TAURI_SIGNING_PRIVATE_KEY` secret) and publishes the release; installed copies offer it on their next start.

The private key lives outside the repository, in `%USERPROFILE%\.tauri\skale-x-updater.key`. Keep a backup: without it no new update can be signed.

---

## Português

**Download:** [Releases](https://github.com/Skale-Club/xcare/releases/latest). O app avisa quando há versão nova e se atualiza sozinho, mantendo as configurações.

O Xcare é um limpador e otimizador portátil no estilo CCleaner e Advanced
SystemCare, mas só com o que funciona de verdade.

- **Visão geral:** nota de 0 a 100 e o que corrigir.
- **Limpeza:** analisa, mostra o tamanho e só apaga o que você marcou, depois
  de confirmar. Logins, histórico, cookies e senhas dos navegadores nunca são
  tocados. Temporários das últimas 24 horas são mantidos.
- **Inicialização:** liga e desliga programas que iniciam com o Windows, com a
  mesma chave do Gerenciador de Tarefas. Dá para desfazer.
- **Programas:** lista com tamanho e abre o desinstalador de cada um.
- **Otimizar:** plano de energia e atalhos para as ferramentas do Windows
  (reparar arquivos do sistema, ponto de restauração, otimizar unidades e mais).
- Para limpar pastas do Windows e mexer em programas de todos os usuários, use
  **Reabrir como administrador**.
- Fora de propósito: limpador de registro, "turbinador de RAM" e "acelerador de
  internet", que não deixam o Windows mais rápido.

## Español

**Descarga:** [Releases](https://github.com/Skale-Club/xcare/releases/latest). La app avisa cuando hay una versión nueva y se actualiza sola, conservando la configuración.

Xcare es un limpiador y optimizador portátil al estilo de CCleaner y Advanced
SystemCare, pero solo con lo que funciona de verdad.

- **Resumen:** nota de 0 a 100 y qué corregir.
- **Limpieza:** analiza, muestra el tamaño y solo borra lo que marques, tras
  confirmar. Las sesiones, el historial, las cookies y las contraseñas nunca se
  tocan. Se conservan los temporales de las últimas 24 horas.
- **Inicio:** activa y desactiva programas que se inician con Windows, con el
  mismo interruptor del Administrador de tareas. Se puede deshacer.
- **Programas:** lista con tamaño y abre el desinstalador de cada uno.
- **Optimizar:** plan de energía y accesos a las herramientas de Windows.
- Para limpiar carpetas de Windows y cambiar programas de todos los usuarios,
  usa **Reabrir como administrador**.
- Fuera a propósito: limpiadores de registro, "aceleradores de RAM" y
  "aceleradores de internet", que no hacen Windows más rápido.

---

© 2026 [Skale Club](https://skale.club)
