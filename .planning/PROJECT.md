# Claket-Tauri

## What This Is

A modern, native soundboard application built with Tauri v2, Vue 3, and Rust. It enables high-performance, low-latency audio playback with precise control over output devices—making it ideal for streamers and power users (e.g., integrating with VoiceMeeter).

## Core Value

Low-latency audio playback with device selection for streaming/VOIP workflows.

## Requirements

### Validated

- ✓ Audio file import (MP3, FLAC, WAV, OGG, AAC) — existing
- ✓ Audio playback with rodio — existing
- ✓ Audio device selection/enumeration via cpal — existing
- ✓ File management (copy to app data directory) — existing
- ✓ Settings persistence via tauri-plugin-store — existing
- ✓ Global keyboard shortcuts — existing
- ✓ Vue 3 + Pinia state management — existing
- ✓ Tailwind CSS v4 + shadcn-vue UI — existing
- ✓ Tauri v2 window management — existing

### Active

- [ ] [To be determined by user]

### Out of Scope

- [Future features to be determined]

## Context

**Technical Environment:**
- Tauri v2 desktop app with Rust backend
- Vue 3 Composition API frontend
- Rodio for audio playback
- cpal for device enumeration
- symphonia for multi-format audio decoding

**Current State:**
- Basic soundboard with audio import, playback, and device selection working
- UI framework established with custom components
- State management via Pinia stores

**Known Issues:**
- Testing not configured (manual verification via `bun run dev`)
- No ESLint/Prettier configured

## Constraints

- **Tech Stack**: Tauri v2 + Vue 3 + Rust — not negotiable
- **Audio Processing**: Must be in Rust (not frontend) — existing architectural decision

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Audio logic in Rust | Native device access via rodio/cpal | ✓ Good |
| Pinia for state | Vue 3 recommended, works well with Tauri | ✓ Good |
| Tailwind CSS v4 | Modern utility CSS, matches shadcn-vue | ✓ Good |

---

*Last updated: 2026-03-24 after project initialization*

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state