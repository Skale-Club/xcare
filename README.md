# Xcare

> **Xcare is now part of [Xpace](https://github.com/Skale-Club/xpace).** Its pages (overview,
> cleaner, startup, apps, optimize and Force Delete) live on in Xpace, next to disk space, drivers
> and hardware. This repository is kept for reference and gets no new features.

Portable PC cleaner and optimizer for Windows, in the spirit of CCleaner and Advanced SystemCare,
with scan-first cleaning and access to Windows maintenance tools. No app installer is required. 🇧🇷
[Português](#português) · 🇪🇸 [Español](#español)

Made by **[Skale Club](https://skale.club)**.

## Availability

**Status checked October 27, 2026:** source version 0.1.1. No GitHub Release or
publicly downloadable executable has been published. Build from source using the
[development guide](docs/DEVELOPMENT.md). Future release binaries belong in
[Releases](https://github.com/Skale-Club/xcare/releases).

The app can offer signed updates when a newer release contains both an executable and its signature.
Updates require internet access and are installed when you select the update action; your `Data`
folder is retained.

## Run it

Double-click `Xcare.exe`. It requires Microsoft Edge WebView2 Runtime. The language selector is at
the bottom left (English, Português, Español) and is saved in `Data\xcare.json` next to the exe.
Keep the executable in a writable folder. The app creates `Data` beside it. PortableApps-style
metadata is optional; the build and release workflow do not create a PortableApps package.

It runs as a normal user. Some Windows folders and programs set up for all users need administrator
rights: use **Restart as administrator** inside the app when you want those too (Windows asks for
permission).

## Pages

| Page | What it does |
|---|---|
| Overview | Checks the PC on open and gives a 0–100 score: junk files, startup programs, free space, time since restart, power plan, drive health, memory. Action buttons are shown where a supported action exists. |
| Cleaner | Scans and shows the size of each category, then deletes only what you ticked, after a confirmation. |
| Startup | Turns programs that start with Windows on and off. |
| Apps | Programs found in supported uninstall registry entries, with size/date when provided; opens the program's own uninstaller. |
| Force Delete | Reviews explicitly selected local files/folders, identifies blocking apps, deletes reviewed items, optionally terminates eligible apps or schedules removal at restart. |
| Optimize | Power plan (including adding Ultimate Performance) and shortcuts to Windows' own tools: repair system files (DISM + SFC), restore point, optimize drives, visual effects, Game Mode, Storage Sense, Windows Update, Task Manager, Disk Cleanup. |

## Cleaning behavior

- Categories cover caches, temporary files and diagnostic leftovers: temp folders,   thumbnail
  cache, error reports, Windows Update downloads, Delivery   Optimization, memory dumps and browser
  caches (Chrome, Edge, Brave, Opera,   Firefox). Browser categories target cache directories, not
  password, cookie or history databases.
- Temp folders keep anything from the last 24 hours, to reduce interference with current work.
- File-based categories delete the scanned files within their allowed roots,   skipping
  links/junctions and files they cannot delete. Close relevant apps   before scanning so caches are
  less likely to change or stay locked.
- Shader caches and the Recycle Bin are off by default. Rebuilding shaders   may delay a game's next
  launch. The Recycle Bin action empties the bin through   Windows at execution time, including
  items added after the scan; it is not a   per-file scan snapshot. Deletion is permanent.
- Clearing browser caches can remove offline cached website resources. The next   visit may need to
  download them again.

## Force Delete

Open **Force Delete** in the sidebar or from **Optimize**. Choose files, choose a folder, drop
local files/folders into the page, or paste an absolute local path. Select **Review selection**
before choosing an action. The tool supports up to 100 selections and 5,000 total items per review.

- **Delete now** permanently removes reviewed items, bypassing the Recycle Bin. Read-only attributes
  can be cleared; ownership and access-control permissions are not rewritten.
- **Terminate apps & delete** explicitly confirms the blocking apps reported by Windows Restart
  Manager. Eligible apps in the current user session can be terminated; unsaved work may be lost.
  Windows processes, services, Explorer, critical processes and Xcare itself are protected. Apps
  are not automatically reopened. Restart Manager does not detect every possible lock.
- **Delete on restart** requires administrator access and registers paths with Windows. It does
  not restart the PC. Registration persists after Xcare closes and does not guarantee removal.
  Windows acts on paths at restart, so a replacement at the same path may also be affected.
  Folder entries are registered after their children and removed only if empty at restart.

Reviews expire after 10 minutes and are consumed by an execution attempt. Immediate deletion checks
file identity and changes since review; new files are not added automatically. Partial results show
deleted, scheduled and failed counts separately. Refresh the review before retrying.

The tool refuses drive roots, network/device paths, symbolic links/junctions, Windows and installed
program trees, and main profile containers. It is intended for stubborn user files and disposable
folders. It does not bypass every Windows restriction. See [implementation and validation
notes](docs/FORCE_DELETE.md).

## What the health score means

The 0–100 score is a heuristic based on the checks in the overview. It is not a benchmark, antivirus
result or guarantee that hardware is healthy. Some findings need manual review rather than a
one-click fix. Power plans and cleanup can change system behavior; the app does not promise a
particular speed increase.

## Startup switch

Turning a program off writes the same "StartupApproved" flag the Task Manager uses. The program's
own entry stays, so you can turn it back on anytime, here or in Task Manager. The list covers
supported Run registry entries and Startup folders; it does not enumerate every service or scheduled
task.

## Left out on purpose

Registry cleaning, forced RAM clearing and automatic network tuning are not implemented. The
Optimize page changes supported power settings or opens Windows tools; availability and elevation
requirements depend on the system.

## Use with Claude Code and Codex

`Xcare.exe --mcp` runs Xcare as a local [MCP](https://modelcontextprotocol.io) server, with no
window, so AI assistants such as Claude Code and Codex can use its engine on this PC. Xcare itself
sends nothing over the network; what the assistant reads goes to its AI provider, as with anything
else you share with it. It only reads: cleaning, startup changes, uninstalling and power plans stay
in the window. Run the assistant as administrator to measure the categories that need it.

Claude Code (available in every project):

```bash
claude mcp add --scope user xcare -- "C:\Apps\Xcare\Xcare.exe" --mcp
```

Codex, in `%USERPROFILE%\.codex\config.toml`:

```toml
[mcp_servers.xcare]
command = 'C:\Apps\Xcare\Xcare.exe'
args = ["--mcp"]
```

Use the path where your `Xcare.exe` is. Tools:

- `app_info`: version, where the exe and `Data` folder are, and the saved settings.
- `status`: system drive space, uptime, memory load, Recycle Bin size, unhealthy drives,
  administrator or not.
- `junk_files`: what each cleaning category would free, in bytes and files; deletes nothing.
- `startup_items`: programs that start with Windows and whether each is enabled.
- `installed_apps`: installed programs with publisher, version, size and date.
- `power_plans`: power plans and the active one.

## Development

Tauri 2, Rust and plain HTML/CSS/JavaScript. See the [development and release
guide](docs/DEVELOPMENT.md) for prerequisites, clone, build, tests and signed-release instructions.
A push to `main` alone does not publish an executable.

---

## Português

**Disponibilidade:** em 27/10/2026, ainda não há release nem executável publicado. Consulte
[Releases](https://github.com/Skale-Club/xcare/releases) ou as [instruções de
compilação](docs/DEVELOPMENT.md). As atualizações dependem de uma release assinada e da ação do
usuário.

O Xcare é um limpador e otimizador portátil no estilo CCleaner e Advanced SystemCare, com análise
antes da limpeza e acesso às ferramentas do Windows.

- **Visão geral:** nota heurística de 0 a 100; não é benchmark, antivírus ou garantia de saúde do
  hardware.
- **Limpeza:** analisa, mostra o tamanho e só apaga o que você marcou, depois   de confirmar. As
  categorias de navegador visam pastas de cache, não os   bancos de senhas, cookies ou histórico. O
  cache offline de sites pode ser   removido. Temporários das últimas 24 horas são mantidos.
- **Inicialização:** liga e desliga programas que iniciam com o Windows, com a   mesma chave do
  Gerenciador de Tarefas. Dá para desfazer.
- **Programas:** lista entradas de desinstalação reconhecidas no registro; tamanho e data dependem
  do instalador. Não é um inventário de todos os apps portáteis ou da Microsoft Store.
- **Lixeira:** a opção é desmarcada por padrão e esvazia a lixeira no momento da execução, inclusive
  itens adicionados após a análise.
- **Exclusão forçada:** revisão de arquivos/pastas locais, identificação de bloqueios, exclusão permanente, encerramento confirmado de programas permitidos e agendamento ao reiniciar (administrador).
- **Otimizar:** plano de energia e atalhos para as ferramentas do Windows   (reparar arquivos do
  sistema, ponto de restauração, otimizar unidades e mais).
- Para limpar pastas do Windows e mexer em programas de todos os usuários, use   **Reabrir como
  administrador**.
- Não inclui limpeza de registro, esvaziamento forçado de RAM ou ajuste   automático de rede. Não
  promete um aumento específico de desempenho.
- **Claude Code e Codex**: `Xcare.exe --mcp` roda o Xcare como servidor MCP local, sem janela, para
  assistentes de IA usarem o diagnóstico do PC (só leitura; limpar e alterar continuam na janela).
  Veja [Use with Claude Code and Codex](#use-with-claude-code-and-codex).

## Español

**Disponibilidad:** al 27/10/2026, aún no hay release ni ejecutable publicado. Consulte
[Releases](https://github.com/Skale-Club/xcare/releases) o las [instrucciones de
compilación](docs/DEVELOPMENT.md). Las actualizaciones requieren una release firmada y la acción del
usuario.

Xcare es un limpiador y optimizador portátil al estilo de CCleaner y Advanced SystemCare, con
análisis antes de limpiar y acceso a las herramientas de Windows.

- **Resumen:** nota orientativa de 0 a 100; no es un benchmark, antivirus ni garantía de salud del
  hardware.
- **Limpieza:** analiza, muestra el tamaño y solo borra lo que marques, tras   confirmar. Las
  categorías de navegador apuntan a carpetas de caché, no a   bases de contraseñas, cookies o
  historial. Puede borrarse la caché offline   de sitios. Se conservan los temporales de las últimas
  24 horas.
- **Inicio:** activa y desactiva programas que se inician con Windows, con el   mismo interruptor
  del Administrador de tareas. Se puede deshacer.
- **Programas:** lista entradas de desinstalación reconocidas en el registro; tamaño y fecha
  dependen del instalador. No es un inventario de todas las apps portátiles o de Microsoft Store.
- **Papelera:** desmarcada por defecto; se vacía en el momento de ejecutar, incluidos elementos
  añadidos después del análisis.
- **Eliminación forzada:** revisión de archivos/carpetas locales, detección de bloqueos, eliminación permanente, terminación confirmada de programas permitidos y programación al reiniciar (administrador).
- **Optimizar:** plan de energía y accesos a las herramientas de Windows.
- Para limpiar carpetas de Windows y cambiar programas de todos los usuarios,   usa **Reabrir como
  administrador**.
- No incluye limpieza del registro, vaciado forzado de RAM ni ajuste   automático de red. No promete
  un aumento concreto del rendimiento.
- **Claude Code y Codex**: `Xcare.exe --mcp` ejecuta Xcare como servidor MCP local, sin ventana,
  para que asistentes de IA usen el diagnóstico del PC (solo lectura; limpiar y cambiar siguen en la
  ventana). Consulta [Use with Claude Code and Codex](#use-with-claude-code-and-codex).

## Skale Club apps

These Windows apps belong to the [Skale-Club organization](https://github.com/Skale-Club).
They share navy interfaces, blue accents, transparent product symbols, and English,
Portuguese and Spanish UI text. Each app has its own executable and portable Data folder.

| App | Purpose |
| --- | --- |
| [Xpace](https://github.com/Skale-Club/xpace) | Care for the PC (disk space, cleaning, drivers, startup, apps) and inspect hardware, live sensors and benchmarks |
| [Xink](https://github.com/Skale-Club/xink) | Draw over the screen during presentations or recordings |
| [Xnote](https://github.com/Skale-Club/xnote) | Transcribe meetings and turn notes into summaries and tasks |
| [Xvert](https://github.com/Skale-Club/xvert) | Convert videos, music and pictures between formats |

Repository access may be required depending on project visibility. Report problems through the
relevant repository, including app version, Windows version and reproduction steps. Remove
personal information from reports and transcripts before sharing.


---

© 2026 [Skale Club](https://skale.club)
