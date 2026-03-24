# Quick Plan: Replace existing aurora background component

**Goal:** Replace existing AuroraBackground.vue with newly installed shadcn-vue inspire-ui aurora version

## Tasks

### Task 1: Replace AuroraBackground.vue with inspire-ui version

**Files:** `src/components/ui/AuroraBackground.vue`

**Action:**
Replace the existing AuroraBackground.vue with the inspire-ui version, adapting it to:
- Use local `cn` from `@/lib/utils` instead of `@inspira-ui/plugins`
- Keep existing Pinia store integration (`useAudioStore`) for reducedMotion
- Keep mode switching (original/optimized) since that's existing functionality

**Done when:** Component compiles and renders with the new inspire-ui aurora effect
