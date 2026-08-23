# Bonk - Developer Agent Guide

## Overview
Bonk is a cross-platform background app that listens to a microphone and maps physical sounds (knocks, claps, snaps) to system actions using pure DSP (no AI/ML).
See [PROJECT.md](./PROJECT.md) for the full, detailed build specification.

## Architecture Map
- `src-tauri/src/main.rs`: App entry and system tray setup
- `src-tauri/src/audio.rs`: Mic capture loop using `cpal`
- `src-tauri/src/dsp.rs`: RMS envelope and FFT feature extraction (`rustfft`)
- `src-tauri/src/classifier.rs`: Distance matching and DTW matching for custom gestures
- `src-tauri/src/pattern.rs`: Multi-hit state machine (e.g., Double Knock)
- `src-tauri/src/actions/`: OS-specific action dispatchers
- `src-tauri/src/config.rs`: JSON serialization for settings and fingerprints
- `src/`: React frontend (Vite, Tailwind, Framer Motion)

## Build and Run
- Desktop Dev: `npm run tauri dev`
- Desktop Build: `npm run tauri build`

## Linters
- Rust: `cargo fmt` and `cargo clippy`
- Frontend: `npm run lint`

## Conventions
- Commits: Maximum 3 words, imperative mood, no prefixes.
- Branching: One branch per phase (`phase-N-description`).
- CI: PRs must pass GitHub Actions CI before self-merging to main.

## Never List
- NO AI/ML logic. All classification is deterministic DSP.
- NO telemetry.
- NO network calls (other than user-defined webhooks).
- NO committing secrets or hardcoded local machine paths.
- NO skipping CI.

## Keep Specs Sync
Keep `PROJECT.md` and `CHANGELOG.md` in sync with reality if scope changes mid-build.
