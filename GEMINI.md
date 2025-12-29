# Claket-Tauri - Gemini Instructions

## Architecture Overview
- **Type**: Native Desktop Soundboard
- **Framework**: Tauri v2 (Rust Backend + Vue 3 Frontend)
- **Audio Engine**: Rust-based (`rodio` + `cpal`) for low latency and multi-output device support.

## Core Logic & Data Flow
1. **Audio Storage**: When a user adds a sound, it is copied to `$APP_DATA/sounds/`. The database (Tauri Store) references these local copies, not the original files.
2. **State Management**: Pinia store (`audio.ts`) handles the UI state, paging, and volume.
3. **Persistence**: `tauri-plugin-store` saves configuration to `store.bin`.
4. **Multi-paging**: Grid of 16 buttons per page. Users can add/rename/delete pages.

## Backend Commands (Rust)
- `list_audio_devices`: Returns available output devices.
- `play_sound`: Triggers audio playback on a specific device with per-button volume.
- `save_sound_file`: Handles the secure copy of external files into the internal library.
- `delete_sound_file`: Removes files from the internal library when a button is reset.

## Critical Implementation Notes
- **Tailwind v4**: Uses the new CSS-first configuration.
- **UI Components**: shadcn-vue.
- **Audio Sync**: Frontend receives `audio-progress` events from Rust to update progress bars in real-time.
- **Windows Focus**: Primary development environment is Windows (PowerShell).

## Interaction Guidelines
- Always use `Bun` for frontend tasks.
- Keep audio processing in Rust to avoid Web Audio API limitations (like output device selection).
- When adding UI features, ensure compatibility with the `AuroraBackground` and existing Glassmorphism/Glow styles.
