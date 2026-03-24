---
phase: quick
plan: 260324-sqb
subsystem: ui
tags: [settings, aurora-background, cleanup]
tech-stack:
  - Vue 3 / Composition API
  - Pinia
  - Tailwind CSS v4
key-files:
  modified:
    - src/stores/audio.ts
    - src/components/ui/aurora-background/AuroraBackground.vue
    - src/App.vue
decisions: []
---

# Quick Task 260324-sqb: Remove Blur Selection in Settings Summary

**One-liner:** Removed Aurora Mode blur selection toggle from settings, hardcoded blur-sm in AuroraBackground component, and cleaned up all related state/UI.

## Tasks Completed

| # | Task | Commit | Status |
|---|------|--------|--------|
| 1 | Remove auroraMode from Pinia store | 35a4f8e | ✅ |
| 2 | Simplify AuroraBackground to hardcode blur-sm | 6f22780 | ✅ |
| 3 | Remove Aurora Mode UI from settings dialog | cf22da3 | ✅ |

## Changes Made

### Task 1: Pinia Store (`src/stores/audio.ts`)
- Removed `auroraMode` state property
- Removed `setAuroraMode()` action
- Removed `auroraMode` persistence in `saveSettings()` and `loadSettings()`

### Task 2: AuroraBackground (`src/components/ui/aurora-background/AuroraBackground.vue`)
- Removed `mode` prop from `AuroraBackgroundProps` interface
- Removed `mode: "optimized"` from `withDefaults`
- Replaced dynamic `:class="props.mode === 'optimized' ? 'blur-sm' : 'blur-xl'"` with static `class="blur-sm"`

### Task 3: Settings Dialog (`src/App.vue`)
- Removed `:mode="store.auroraMode"` prop binding from `<AuroraBackground>`
- Removed entire Aurora Mode section (Blur 4px / Blur 10px buttons)
- Removed unused `Zap` and `Sparkles` imports from lucide-vue-next

## Verification

- ✅ TypeScript compiles cleanly (`npx vue-tsc --noEmit` — no errors)
- ✅ No remaining references to `auroraMode` or `setAuroraMode` in codebase
- ✅ Accent Color and Reduced Motion settings unaffected

## Deviations from Plan

None — plan executed exactly as written.

## Self-Check: PASSED

All files modified, all commits exist, TypeScript clean.
