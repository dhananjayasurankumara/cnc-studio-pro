# CNC Studio Pro: one-click Windows 11 installer

The GitHub Actions workflow compiles the app on a Windows runner. **You do not need Node.js, Rust, or build tools on the Windows 11 PC where you install the resulting app.**

1. Sign in to GitHub and create a **private** repository (e.g. `cnc-studio-pro`).
2. Upload all files **inside** the `cnc-studio-pro` directory to the repository root, including the hidden `.github/workflows/windows-installer.yml` file. The easiest way to preserve hidden files is using GitHub Desktop to commit and publish the extracted directory.
3. Open the repository on github.com and select **Actions** → **Build Windows 11 installer** → **Run workflow**.
4. Wait until the workflow completes with a green checkmark. Open the run and download the artifact named **CNC-Studio-Pro-Windows-11-Setup** at the bottom of the run page.
5. Unzip that artifact. Double-click the `.exe` installer to install CNC Studio Pro.

GitHub Actions build minutes and artifact retention depend on your GitHub plan. The workflow does not publish your source or installers publicly by itself. Windows SmartScreen may warn about an unsigned application; commercial distribution requires code signing.

**Build status:** The GitHub workflow and app have not been compiled or verified in a Windows environment here. A red workflow run means the project needs debugging; no working installer exists until the build succeeds.

**Machine safety:** Before enabling motion, verify wiring, motor current, axis direction, limit switches, and a reachable physical emergency stop. Test with the pen raised and motors unloaded. Stock GRBL 1.1 does not directly implement generic servo-angle pen-lift commands; use verified servo-capable firmware and matching pen commands.
