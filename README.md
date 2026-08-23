# Bonk

**A cross-platform background app that turns physical sounds (knocks, claps, slams, snaps) into customizable system actions.**
No AI. No ML. Pure DSP and 100% local.

![Logo](bonk.png)

## Features
- **11 Built-in Gestures:** Single/Double/Triple Knock, Slam, Clap, Snap, Whistle, and more.
- **Custom Gestures:** Record your own tap patterns or meme sounds.
- **Action Bank:** Lock screen, `git push`, open apps, trigger webhooks, control media, and more.
- **Action Chains:** Run multiple actions in sequence from a single trigger.

## Installation
*(Installer downloads coming soon)*

### Note for macOS and Windows Users (Unsigned App)
Bonk is an indie open-source app and is not currently signed with an Apple Developer ID or Windows Authenticode certificate. 
You may see a Gatekeeper or SmartScreen warning. Bonk is 100% local, operates strictly offline (unless you configure webhooks), and its source code is fully auditable.
- **Windows:** Click "More info" -> "Run anyway".
- **macOS:** Go to System Settings -> Privacy & Security -> Open Anyway.

### Microphone Permissions
Bonk requires microphone access to detect sounds. It operates entirely locally and does not record or transmit audio.

## Quick Start
1. Calibrate a gesture in the wizard (e.g., Knock 3 times).
2. Map it to an action (e.g., Lock Screen).
3. That's it! Tap your desk to trigger it.

## Building from Source
1. Install Node.js (v20+) and Rust.
2. Clone the repo and run `npm install`.
3. Start development server: `npm run tauri dev`.

## License
Licensed under the [PolyForm Noncommercial 1.0.0](LICENSE). 
Free to use, modify, and self-host for non-commercial purposes. See [CONTRIBUTING.md](CONTRIBUTING.md) for commercial licensing inquiries.
