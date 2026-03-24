---
phase: quick
plan: 260324-sqb
type: execute
wave: 1
depends_on: []
files_modified:
  - src/components/ui/aurora-background/AuroraBackground.vue
  - src/App.vue
  - src/stores/audio.ts
autonomous: true
requirements: []

must_haves:
  truths:
    - "Settings dialog no longer shows Aurora Mode selection"
    - "Aurora background always uses blur-sm (optimized mode)"
    - "accentColor and reducedMotion settings are preserved"
    - "No TypeScript or Rust LSP errors"
  artifacts:
    - path: "src/components/ui/aurora-background/AuroraBackground.vue"
      provides: "Aurora background with hardcoded blur-sm"
    - path: "src/App.vue"
      provides: "Settings dialog without Aurora Mode section"
    - path: "src/stores/audio.ts"
      provides: "Pinia store without auroraMode state/action"
  key_links:
    - from: "App.vue"
      to: "AuroraBackground.vue"
      via: "component props"
      pattern: ":mode="
---

## Objective

Remove the Aurora Mode blur selection from settings and hardcode the optimized (blur-sm) mode in AuroraBackground. Simplifies the UI by removing an unnecessary setting.

**Purpose:** Eliminate unused blur mode toggle — only blur-sm is needed.
**Output:** Clean settings dialog, simplified AuroraBackground component, trimmed Pinia store.

## Context

AuroraBackground currently has a `mode` prop ("original" | "optimized") that toggles between `blur-xl` (24px) and `blur-sm` (4px). The settings dialog exposes two buttons to select this. Since only "optimized" (blur-sm) is desired, we remove the entire selection mechanism and hardcode `blur-sm`.

## Tasks

<task type="auto">
  <name>Task 1: Remove auroraMode from Pinia store</name>
  <files>src/stores/audio.ts</files>
  <action>
    Remove the `auroraMode` state property and `setAuroraMode` action from the audio store. Also remove persistence of `auroraMode` in both `saveSettings()` and `loadSettings()`.

    Specific removals:
    - Line 63: Remove `auroraMode: "optimized" as "original" | "optimized",` from state
    - Lines 441-444: Remove the `setAuroraMode` action
    - Line 535: Remove `await store.set("auroraMode", this.auroraMode);` from saveSettings
    - Line 553: Remove `const savedAuroraMode = await store.get<string>("auroraMode");` from loadSettings
    - Lines 573-575: Remove the `if (savedAuroraMode ...)` block from loadSettings
  </action>
  <verify>
    <automated>npx vue-tsc --noEmit 2>&1 | head -20</automated>
  </verify>
  <done>auroraMode state and setAuroraMode action removed, no TypeScript errors referencing auroraMode</done>
</task>

<task type="auto">
  <name>Task 2: Simplify AuroraBackground to hardcode blur-sm</name>
  <files>src/components/ui/aurora-background/AuroraBackground.vue</files>
  <action>
    Remove the `mode` prop from AuroraBackgroundProps interface and defaults. Replace the dynamic blur class with hardcoded `blur-sm`.

    Specific changes:
    - Remove `mode?: "original" | "optimized";` from AuroraBackgroundProps (line 9)
    - Remove `mode: "optimized",` from withDefaults (line 17)
    - Replace line 87: `:class="props.mode === 'optimized' ? 'blur-sm' : 'blur-xl'"` → `class="blur-sm"`
  </action>
  <verify>
    <automated>npx vue-tsc --noEmit 2>&1 | head -20</automated>
  </verify>
  <done>mode prop removed, blur-sm hardcoded, no TypeScript errors</done>
</task>

<task type="auto">
  <name>Task 3: Remove Aurora Mode UI from settings dialog</name>
  <files>src/App.vue</files>
  <action>
    Remove the Aurora Mode selection section from the settings dialog and the `:mode` prop binding on AuroraBackground.

    Specific changes:
    - Line 213: Remove `:mode="store.auroraMode"` from the `<AuroraBackground>` component
    - Lines 279-348: Remove the entire `<div class="space-y-3">` block containing "Aurora Mode" heading and the two blur selection buttons
    - Remove unused lucide imports `Zap` and `Sparkles` from the import on line 31 (they're only used in the Aurora Mode buttons)
  </action>
  <verify>
    <automated>npx vue-tsc --noEmit 2>&1 | head -20</automated>
  </verify>
  <done>Aurora Mode UI removed from settings, :mode prop removed from AuroraBackground, unused imports cleaned</done>
</task>

## Verification

```bash
# TypeScript check
npx vue-tsc --noEmit

# Build check
bun run build
```

## Success Criteria

- [ ] Settings dialog has no Aurora Mode section
- [ ] AuroraBackground always renders with blur-sm
- [ ] `store.auroraMode` is no longer referenced anywhere
- [ ] TypeScript compiles cleanly
- [ ] Accent Color and Reduced Motion settings still work

## Output

After completion, update `.planning/STATE.md` with completed quick task entry.
