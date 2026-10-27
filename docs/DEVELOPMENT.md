# Development and releases — Xcare

This guide is for maintainers building, checking and publishing Xcare for Windows. For everyday use,
return to the [README](../README.md).

## Build locally

Use Windows with Microsoft Edge WebView2 Runtime, Node.js/npm, Rust's MSVC toolchain and Microsoft
C++ Build Tools with the Windows SDK. Install the **Desktop development with C++** workload. The
repository's release workflow currently selects Node.js 20 and stable Rust. See the [Tauri Windows
prerequisites](https://v2.tauri.app/start/prerequisites/#windows) for setup details.

From a directory where you keep source repositories:

~~~powershell
git clone https://github.com/Skale-Club/xcare.git
cd xcare
npm ci
npm run dev
~~~

Private repositories require an authenticated GitHub account with access. The frontend is plain
HTML, CSS and JavaScript served by Tauri 2; opening the HTML directly in a browser does not provide
the native commands.

To check Rust code, run the included tests and build the executable:

~~~powershell
cargo check --manifest-path src-tauri/Cargo.toml --locked
cargo test --manifest-path src-tauri/Cargo.toml --release --locked
npm run build
~~~

The build produces `src-tauri/target/release/xcare.exe`. It does not create an installer or a
PortableApps package. Copy the executable to a writable folder for manual testing. Its `Data` folder
belongs beside that executable, including when running a development build.

## Architecture

Rust handles scan snapshots, deletion boundaries, registry startup flags, installed-program entries
and Windows maintenance commands. JavaScript renders scan progress, user confirmations and the
heuristic health score. Keep scan and deletion separate; do not turn viewing a page into a
destructive action.

All four apps use the same palette: navy surfaces, primary blue `#5173D6` and soft blue `#8FA9EE`.
Keep English, Portuguese and Spanish UI strings consistent. Settings and application data use the
portable data directory; do not commit runtime data, API credentials or private signing keys.

## Validate a change

Run the checks above when changing application code. Unit tests do not validate real Windows
devices, capture software or external APIs. For documentation-only changes, compare claims with the
implementation and check links and commands.

- Review the overview, scan categories and cancellation/confirmation flows.
- Test deletion only with disposable files in a controlled Windows environment.
- Confirm recent temporary files, reparse points and files outside category roots are protected.
- Verify a disposable startup entry can be disabled and re-enabled.
- Check that uninstall and maintenance buttons open the intended Windows tools; do not test them on
  valuable installed software.

Keep documentation aligned with observed behavior. Update the README's English instructions and
Portuguese/Spanish summaries together. A feature listed in the README should have an implemented
workflow; a release link should not imply that a binary exists before publication succeeds.

## Publish a release

The **Release** GitHub Actions workflow runs when a `v*` tag is pushed. A normal branch push does
not build or publish an executable. Publication needs repository write access, GitHub Actions
enabled and an Actions secret named `TAURI_SIGNING_PRIVATE_KEY` available to this repository. An
organization secret must grant this repository access.

The secret must contain the private signing key matching the app's embedded public update key. Keep
the key outside Git and back it up securely. The current workflow supplies an empty signing-key
password; using a password-protected key requires updating the workflow's secret configuration
first.

**Readiness checked October 26, 2026:** no releases or repository-level Actions secrets were listed
for this app. Confirm that a matching signing key is available through a repository or permitted
organization secret before pushing a release tag.

For a reviewed release:

1. Start from a clean checkout of `main`, fetch the remote and confirm `origin`    points to
   `https://github.com/Skale-Club/xcare.git`.
2. Choose an unused semantic version. Set the same version in the npm package,    Rust package and
   Tauri configuration. Refresh the npm lockfile with    `npm install --package-lock-only` and the
   Rust lockfile with    `cargo check --manifest-path src-tauri/Cargo.toml`. Review those changes.
3. Run the tests and build above, then perform the relevant manual checks.
4. Commit the version changes and lockfiles. Create the corresponding `vMAJOR.MINOR.PATCH`    tag on
   that commit, then push `main` and that exact tag to `origin`.
5. Watch the [Actions run](https://github.com/Skale-Club/xcare/actions).    The tag must match the
   Rust package version. The workflow builds the app,    signs it and publishes `Xcare.exe` plus
   `Xcare.exe.sig` in    [Releases](https://github.com/Skale-Club/xcare/releases).
6. Download and smoke-test the release. Confirm both assets are present and    readable by the
   intended users before announcing a download or update.    Replace the dated pre-release notice in
   the README after the first release.

The existing `scripts/release.ps1` helper changes the three version manifests, commits, tags and
pushes immediately. It does not refresh lockfiles, run tests or check that signing secrets exist.
Use the reviewed process above when preparing lockfiles or validating a first release.

The updater queries GitHub without authentication, finds a newer version, verifies its signature and
replaces the executable after the user selects the update. It retains `Data`. Release assets must be
publicly readable for this updater to work; GitHub login in a browser does not authenticate the app.

The update signature is separate from Windows Authenticode signing; this workflow does not configure
an Authenticode certificate.

## Portable distribution

The app creates `Data` next to its executable. Optional PortableApps-style metadata can be supplied
under `App/AppInfo`; the app only refreshes the version when that metadata already exists. Neither
the build nor the release workflow creates a complete PortableApps package.

Close the app before moving or backing up its folder.
