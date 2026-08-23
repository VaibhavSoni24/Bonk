# Bonk - Project Specification

## 1. Overview & Philosophy
**One-liner:** A cross-platform background app that listens to your microphone and turns physical sounds (knocks, claps, slams, snaps) into customizable system actions.

**Core Principles:**
- **Zero AI / ML:** No LLM calls, no trained models, no cloud inference. All classification is done via deterministic DSP (amplitude envelopes, FFT, spectral centroid/spread, and DTW matching against calibrated fingerprints). 
- **100% Local & Private:** Zero API keys, zero cloud dependency, zero telemetry. The app never touches the network unless explicitly configured to do so via user-defined webhooks/URLs.
- **Small and Light:** A tray app should never feel like a heavy browser. 
  - *Target Size Budget:* < 30MB installed binary.
  - *Target Idle RAM:* < 50MB.
  - *Target Idle CPU:* < 1-2%.
- **Cross-Platform:** Full feature parity across Windows, macOS, and Linux from a single codebase.
- **Accuracy Above All:** The gesture classifier relies on robust confidence thresholds, clear calibration flows, and strict false-positive rejection.
- **Frictionless Onboarding:** Install → Open → (Optional Calibration) → Done. Built-in gestures have sensible stock mappings out-of-the-box.
- **Licensing:** Licensed under **PolyForm Noncommercial 1.0.0**. The source is public and auditable, reinforcing trust. Free for non-commercial use.

## 2. Platform & Tech Stack
**App Shell & Backend: Tauri + Rust**
Tauri replaces Electron to dramatically reduce the memory footprint and binary size by using the OS's native webview. Rust provides a highly performant, safe backend for continuous audio processing without blowing the CPU budget.

**Audio Capture: `cpal`**
Cross-platform low-latency audio I/O crate for Rust. Captures continuous mic input in a background thread independent of the UI state.

**DSP & FFT: `rustfft`**
Fast Fourier Transform library for Rust. Used to extract frequency profiles and spectral centroids from captured transients.

**Frontend: React + Tailwind CSS + Framer Motion**
React provides a component-based architecture for the UI. Tailwind CSS enables a strict, maintainable dark-theme design system. Framer Motion provides the premium, purposeful animations required for the live waveform and calibration wizard.

## 3. Full Gesture Roster
The built-in roster consists of 11 distinct gestures, plus support for custom gestures.

| # | Gesture | Signature | Detection Basis |
|---|---|---|---|
| 1 | **Single Knock** | Short spike (~50-150ms), mid-low freq (200-800Hz) | Baseline transient |
| 2 | **Double Knock** | Two knock events within ~150-500ms | Pattern layer (reuses knock fingerprint) |
| 3 | **Triple Knock** | Three knock events within ~150-700ms | Pattern layer |
| 4 | **Slam** | Very high amplitude, low freq (<400Hz), thud shape | Amplitude-dominant classification |
| 5 | **Clap** | Sharp spike (~20-80ms), high freq content (2k-8kHz) | High spectral centroid vs knock |
| 6 | **Double Clap** | Two clap events within ~150-500ms | Pattern layer (reuses clap fingerprint) |
| 7 | **Triple Clap** | Three clap events within ~150-700ms | Pattern layer |
| 8 | **Snap** | Very short (~10-40ms), very high freq (4kHz+) | Shortest duration + highest centroid |
| 9 | **Double Snap** | Two snap events within ~150-500ms | Pattern layer (reuses snap fingerprint) |
| 10| **Whistle** | Sustained narrow-band tone | Tonal/periodic distinction, narrow spectral spread |
| 11| **Long Scrape** | Sustained (600ms+), broadband continuous energy | Duration-dominant classification (plateau) |

## 4. Custom Gesture Engine
Users can record custom gestures ("meme sounds", specific tap sequences on objects, spoken words).
- **Cap:** Maximum 10 custom gestures per user.
- **Recording Length:** Hard cap of **2.0 seconds**, minimum **0.3 seconds**.
- **Pattern Layer:** Single-shot only for v1 (no "double custom" combos).

**DSP Matching Approach (DTW):**
1. Slice the recorded 0.3s–2.0s template into short frames (e.g., 10-20ms).
2. Extract a lightweight per-frame feature vector: **RMS energy + spectral centroid**.
3. Store this as a small array of frames (tens of KB max).
4. At runtime, when a sustained/complex event is detected, extract the same features from live audio.
5. Match against custom templates using **Dynamic Time Warping (DTW)**, allowing for slight timing variations.
6. Accept match if the DTW distance is below a strict confidence threshold.

## 5. Full Action Bank
All actions are implemented natively per OS, maintaining cross-platform parity. 

### Screen & Capture
- **Screenshot (full screen):** Save to configurable folder.
- **Screenshot (active window only):**
- **Toggle Screen Recording:**
- **Toggle Webcam:** Best effort via device enable/disable or app-level capture mute.

### System State
- **Lock screen:** 
  - *Windows:* `user32.dll LockWorkStation`
  - *macOS:* CGSession/AppleScript
  - *Linux:* `loginctl lock-session` or `xdg-screensaver`
- **Sleep:** Suspend to RAM.
- **Toggle Do Not Disturb / Focus Mode:**
- **Toggle Mic:** Mute/unmute default input device.
- **Toggle System Volume Mute:** 
- **Pause/Resume:** Media play/pause key simulation.
  - *Windows:* `keybd_event` or SendInput.
  - *macOS:* AppleScript / Media key event.
  - *Linux:* `xdotool` or MPRIS D-Bus.
- **Toggle Hotspot:** 
  - *Windows:* Windows Runtime API (`Windows.Networking.NetworkOperators`).
  - *macOS/Linux:* Best-effort via shell/`nmcli`, often requires elevation.
- **Toggle Battery Saver:** 
  - *Windows:* PowerSetting APIs.
  - *macOS/Linux:* Best-effort via `pmset` or `upower`, often requires elevation.
- **Toggle Bluetooth / WiFi / Airplane Mode:**
  - *Note on Unix (macOS/Linux):* These hardware toggles often require elevated privileges. The app will make a "best-effort" call, prompting for elevation (e.g., `pkexec` on Linux or standard macOS auth dialog) when required. Document limitations in the UI if an OS blocks third-party toggling entirely.

### App & File Launchers
- **Open Browser:** URL or default homepage.
- **Open Terminal:** Default OS terminal.
- **Open File Manager:** Explorer / Finder / Nautilus.
- **Open OS Settings:** 
- **Open specific app / file:** User-browsed executable path.
- **Custom Action Chain:** A sequential list of actions (e.g., Open Finder -> Open Terminal). Managed via a **simple ordered-list UI builder** (add step, pick action + params, reorder up/down). 

### Dev Workflow
- **`git push`:** Runs against an explicitly user-configured, pinned repository path.
- **`git pull`:** Runs against the configured path.
- **Run custom shell command/script:** Free text field escape hatch.
- **Run configured build/test command:**

### Window Management
- **Minimize all windows / show desktop:**
- **Snap active window left/right:**
- **Switch to next virtual desktop / Space:**
- **Close active window:**

### Fun / Meme Tier
- **Play a sound effect:** User-selected local audio file (.wav/.mp3).
- **Send a webhook:** Configurable URL + JSON payload (Slack/Discord compatible).
- **Screenshot + auto-copy to clipboard:**

*A "None / Disabled" option is always available for unused gesture slots.*

## 6. Architecture — Layer by Layer
1. **Capture Layer (cpal):** Continuous background thread listening to default mic. Short buffers (10-30ms).
2. **Envelope Layer:** Calculates short-time RMS amplitude. Events begin when RMS crosses a user-calibrated noise floor threshold, ending when it drops below the threshold (with a brief grace period to handle micro-gaps, especially for Scrapes).
3. **FFT / Spectral Layer (rustfft):** Run on the captured event window to extract spectral centroid (brightness) and spectral spread.
4. **Fingerprint Storage:** Saved in local JSON config (appData dir).
5. **Distance Classifier:** Compares normalized incoming transient features (duration, peak amplitude, centroid) against the 11 built-in fingerprints. Matches the closest one if within confidence thresholds; else rejects as noise.
6. **DTW Matcher (Custom Gestures):** If the event length and complexity matches custom parameters, frame-wise features are compared via DTW against custom templates.
7. **Pattern Layer:** A state machine tracking timing windows (~150-700ms) to upgrade single-hits (Knock) to multi-hits (Double Knock, Triple Knock).
8. **Action Dispatcher:** Rust backend executes the OS-specific command/API mapped to the detected gesture.
9. **UI Layer:** Tauri WebView (React) talking to the Rust backend via Tauri IPC for live metering, settings updates, and calibration workflows.

## 7. Data Models

```json
// App Config Schema (config.json)
{
  "settings": {
    "launchOnStartup": true,
    "micDeviceId": "default",
    "noiseFloorThreshold": 0.05,
    "confidenceThreshold": 0.8
  },
  "fingerprints": {
    "Knock": { "avgDuration_ms": 70, "avgAmplitude": 0.6, "avgCentroid_hz": 400, "durationRange": [40,120] },
    // ... other built-ins
  },
  "customTemplates": [
    {
      "id": "custom_1",
      "name": "Meme Sound",
      "frames": [
        { "rms": 0.5, "centroid": 1200 },
        { "rms": 0.6, "centroid": 1300 }
      ]
    }
  ],
  "mappings": {
    "SingleKnock": { "type": "screenshot" },
    "DoubleKnock": { "type": "webhook", "params": { "url": "https://..." } },
    "Custom_1": {
      "type": "action_chain",
      "steps": [
        { "type": "open_folder", "params": { "path": "/Users/dev/repo" } },
        { "type": "git_push", "params": { "path": "/Users/dev/repo" } }
      ]
    }
  }
}
```

## 8. Installer & Distribution Spec
- **Code Signing:** **Unsigned for v1.** A conscious tradeoff.
- **First-Run / README Warning:** Must include plain English documentation explaining that Bonk is 100% local, open-source under PolyForm Noncommercial 1.0.0, and how to safely bypass Windows SmartScreen and macOS Gatekeeper warnings for unsigned indie apps.
- **Windows:** `.exe` wizard installer (Tauri bundler). User picks install destination.
- **macOS:** `.dmg` (Tauri bundler).
- **Linux:** `.AppImage` and `.deb` (Tauri bundler).
- **Autostart:** Defaults to ON. Configured via OS-specific mechanisms (Registry on Windows, LaunchAgent on macOS, `.desktop` file in `~/.config/autostart/` on Linux).
- **Uninstaller:** Clean uninstallation removing registry/autostart entries, avoiding orphaned background tasks.
- **Auto-Updater:** Not included in v1. Manual download for updates.

## 9. UI/UX Spec
- **Theme:** Premium Dark UI. 
- **Palette:** Anchored by the logo (`bonk.png` - dark navy/black + cyan burst). 
  - Backgrounds: Dark neutral/navy scale.
  - Accent: Cyan (active states, primary buttons, live waveform, calibration progress). No competing accent colors.
- **Motion:** Framer Motion used for smooth state transitions, hover effects, and a responsive live mic waveform/level meter.
- **Assets Pipeline:** 
  1. Primary Logo (`bonk.png`) for About, Splash, README.
  2. Derived simplified low-detail variant for tray/taskbar icons (16-32px).
  3. Derived near-monochrome "template" variant for the macOS menu bar.

**Core Screens:**
1. **Tray Presence:** Icon + Right-click menu (Open, Pause Listening, Quit). Left click opens UI.
2. **Onboarding / Permissions:** Short explainer shown before the OS mic-permission prompt, especially framed for macOS's stricter flow to ensure users understand the local-only nature before accepting.
3. **Dashboard:** Live mic level meter, Listening toggle, Recent Activity Log (last 10 triggers).
4. **Calibration Wizard:** Step-by-step guided recording (3 reps for built-ins, 1 rep for custom). Visual waveform + countdown.
5. **Gesture Mapping:** Table of all 11 built-ins + up to 10 customs. Searchable action dropdowns. Expandable inline fields for params. Includes the **Action Chain Builder** (ordered list UI).
6. **Settings:** Launch on startup, noise floor slider, confidence slider, mic selector, About/License info.

## 10. File & Project Structure
Standard Tauri monorepo structure:
```text
bonk/
├── src-tauri/             # Rust backend
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs        # App entry, Tray setup
│   │   ├── audio.rs       # cpal mic capture loop
│   │   ├── dsp.rs         # rustfft, envelope calculation
│   │   ├── classifier.rs  # Distance matching & DTW
│   │   ├── pattern.rs     # Multi-hit state machine
│   │   ├── actions/       # OS-specific dispatchers
│   │   └── config.rs      # JSON serialization
├── src/                   # React frontend
│   ├── App.tsx
│   ├── components/        # Wizard, Dropdowns, ActionChainBuilder
│   └── styles/            # Tailwind configuration
├── package.json
├── PROJECT.md             # This spec
├── README.md              # Documentation + SmartScreen bypass guide + License summary
└── LICENSE                # PolyForm Noncommercial 1.0.0
```

## 11. Build Order (Single Pass)
1. **Backend Scaffolding:** Scaffold Tauri + React. Setup system tray and background thread.
2. **Audio Capture:** `cpal` integration. Ensure raw PCM buffer is captured cross-platform.
3. **DSP Pipeline:** Implement RMS envelope detection and event slicing.
4. **FFT & Feature Extraction:** Integrate `rustfft`. Log outputs offline to validate.
5. **Classifier Engine:** Implement distance matching for built-ins and DTW for custom gestures.
6. **Pattern Layer:** Implement the time-window state machine for double/triple hits.
7. **Action Dispatchers:** Implement OS-specific commands (start with simple ones like Lock Screen and Shell execution).
8. **Frontend Wiring:** Connect live mic data and config to React via Tauri IPC.
9. **UI Implementation:** Dashboard, Calibration Wizard, and Action Mapping (including Action Chain builder).
10. **Distribution Prep:** Configure Tauri bundler for `.exe`, `.dmg`, `.deb`, `.AppImage`.

## 12. Known Tricky Bits
- **Slam vs Clap Confusion:** Slam is low-freq, clap is high-freq. Rely heavily on spectral centroid, not just amplitude.
- **Room Noise Variance:** Quiet vs noisy rooms require the user-adjustable noise floor slider to be prominent.
- **Scrape Grace Period:** Micro-gaps in physical scraping can drop the envelope. A ~50ms grace period before finalizing an event is required.
- **Mac Mic Permissions:** macOS strictly requires `Info.plist` strings and often requires manual system settings approval. The app must handle denial gracefully and guide the user.
- **Hardware Toggles on Unix:** Prompting for `pkexec`/sudo correctly without hanging the background thread.
- **DTW Thresholds:** Tuning the DTW distance confidence threshold so a custom phrase doesn't randomly trigger on coughs.

## 13. Accuracy & Testing Strategy
- **Headless Validation:** Before hooking up the UI, the classifier must be tested locally. Log feature values (RMS, centroid) to standard out for 20 reps of every gesture to ensure distinct, non-overlapping clusters.
- **Confusion Checking:** Validate that Triple Knock doesn't prematurely trigger Double Knock. Ensure the state machine correctly waits for the pattern window to expire.
