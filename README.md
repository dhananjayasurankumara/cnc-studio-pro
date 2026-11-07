# CNC Studio Pro — Tauri 2 / React / TypeScript / Rust

An **early-stage desktop starter**, not a production-ready CNC controller. Includes a dark desktop UI, Windows Tauri packaging, native serial port enumeration, GRBL serial I/O, basic status reporting, X/Y `$J` jogging, motor channel tests, editable servo G-code commands, a simple G0/G1 XY preview, and a conservative one-command-at-a-time G-code sender.

## Windows prerequisites

1. Install **Node.js LTS** (with npm), **Rust** (rustup, MSVC toolchain), **Microsoft C++ Build Tools** with Desktop development with C++, and **Microsoft Edge WebView2 Runtime**. For MSI packaging you may also need Windows Installer tooling supported by Tauri.
2. Unzip the project and open PowerShell in the `cnc-studio-pro` folder.
3. Run `npm install`.
4. Run `npm run tauri dev` for a desktop development window.
5. Run `npm run tauri build` for Windows installers. Installers should appear under `src-tauri/target/release/bundle/nsis/` and `.../msi/`.

Official setup: https://v2.tauri.app/start/prerequisites/

## First machine connection

1. Flash compatible GRBL 1.1 to your Arduino UNO separately using the Arduino IDE or a trusted flasher. Firmware flashing is **not** implemented in this starter.
2. Attach CNC Shield V3, properly configured A4988 drivers, motors, and **separate motor power**. Do not connect/disconnect motors with driver power on.
3. Connect Arduino USB, open the app, click **Ports**, select COM port and **115200**, then **Connect Arduino**.
4. Read `$$` and `$I` from Settings. Ensure GRBL is idle and the machine can move safely.
5. In Manual Control select **0.1 mm** and a conservative feed; test X and Y separately.
6. Test with the pen raised, no obstacles, and a physical emergency-stop within reach.

## Pen servo / firmware

**Stock GRBL 1.1 on UNO does not directly drive a hobby pen-lift servo.** The UI's editable pen commands default to `M5` and `M3 S30`, but these are merely example spindle-style commands and are NOT guaranteed servo commands. Use a firmware fork that explicitly supports servo lifting and configure the commands for that fork. Power servos correctly and share ground where appropriate. No servo angle feedback is available.

## What is implemented

- Rust serial backend: port listing, open/close, byte sending, soft reset, line events
- GRBL response/status console; periodic `?` polling
- XY jogging via `$J=G91 G21 ...`, feed hold `!`, resume `~`, jog cancel `0x85`
- X/Y channel tests via jog; no automatic motor identification
- Editable G-code with file import/export and G0/G1 2D preview
- G-code streaming with **one command outstanding**, awaiting `ok` before sending the next
- Local simulation slider and basic job progress

## Important limitations and next steps

- The **streamer is a starter**: it doesn't maintain an exact controller planner/buffer model, handle robust reconnects, prove motion completion, or fully coordinate feed hold / reset / alarm / jog cancel. It is not suitable for unattended jobs. `ok` means the line was accepted, not that the movement completed.
- **Pause/resume and abort** should be hardened with an explicit state machine, GRBL idle confirmation, and disconnection handling before production use. Abort currently sends feed hold; it does not forcibly power down motors. For emergency stopping, use hardware E-stop.
- The preview supports only straight XY G0/G1 moves. It does not handle G2/G3 arcs, G20 inch conversion, work offsets, G92, coordinate systems, or full modal interpretation.
- No SVG/DXF/image tracing, text-to-G-code, firmware flashing, automatic calibration, full 3D simulation, persistence, or robust queued job recovery yet.
- `$$` configuration is read-only through the console; no automated calibration routine is implemented.
- The UI uses an online Inter font when available but falls back to Segoe UI offline.
- **This archive has not been compiled or hardware-tested in this environment** because dependency installation requires network access and a Windows/Rust build environment.

## Project structure

```
src/App.tsx                    React UI, GRBL events, job sender, jog controls, preview
src/index.css                 Tailwind and custom dark design system
src-tauri/src/lib.rs          Rust serial port implementation and Tauri commands
src-tauri/tauri.conf.json     Desktop window and installer config
src-tauri/capabilities/       Tauri permission capabilities
```

## Safety

Disconnect motor power before changing wiring. Use suitable current limits and external motor supply. Servo support depends on firmware and wiring. Keep travel within machine limits, verify GRBL settings and limit switches, and supervise all motion. A physical emergency stop is essential.
