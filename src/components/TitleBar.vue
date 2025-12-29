<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, Square, X, Copy } from "lucide-vue-next";
import { ref, onMounted, onUnmounted } from "vue";
import { useAudioStore } from "../stores/audio";

const store = useAudioStore();
const appWindow = getCurrentWindow();
const isMaximized = ref(false);
let unlisten: (() => void) | null = null;

const minimize = () => appWindow.minimize();
const toggleMaximize = async () => {
  await appWindow.toggleMaximize();
  isMaximized.value = await appWindow.isMaximized();
};
const close = () => appWindow.close();

const handleMouseDown = (e: MouseEvent) => {
  if (e.buttons === 1) { // Left click
    if (e.detail === 2) { // Double click
      toggleMaximize();
    } else {
      appWindow.startDragging();
    }
  }
};

onMounted(async () => {
  isMaximized.value = await appWindow.isMaximized();
  
  // Listen to resize events to update isMaximized state
  unlisten = await appWindow.onResized(async () => {
    isMaximized.value = await appWindow.isMaximized();
  });
});

onUnmounted(() => {
  if (unlisten) unlisten();
});
</script>

<template>
  <div 
    @mousedown="handleMouseDown"
    class="h-8 flex items-center bg-transparent select-none fixed top-0 left-0 right-0 z-[100] cursor-default px-1"
    :class="store.controlsSide === 'left' ? 'flex-row' : 'flex-row-reverse'"
  >
    <!-- Controls Container -->
    <div class="flex h-full items-center">
      <!-- Mac Style -->
      <div v-if="store.titlebarStyle === 'mac'" class="flex items-center px-3 gap-2 h-full group">
        <button
          @click="close"
          @mousedown.stop
          class="w-3 h-3 rounded-full bg-[#ff5f57] border border-black/10 flex items-center justify-center transition-all"
        >
          <X :size="8" class="text-black/60 opacity-0 group-hover:opacity-100" />
        </button>
        <button
          @click="minimize"
          @mousedown.stop
          class="w-3 h-3 rounded-full bg-[#ffbd2e] border border-black/10 flex items-center justify-center transition-all"
        >
          <Minus :size="8" class="text-black/60 opacity-0 group-hover:opacity-100" />
        </button>
        <button
          @click="toggleMaximize"
          @mousedown.stop
          class="w-3 h-3 rounded-full bg-[#28c940] border border-black/10 flex items-center justify-center transition-all"
        >
          <Copy v-if="isMaximized" :size="6" class="text-black/60 opacity-0 group-hover:opacity-100" />
          <Square v-else :size="6" class="text-black/60 opacity-0 group-hover:opacity-100" />
        </button>
      </div>

      <!-- Windows Style -->
      <div v-else class="flex h-full">
        <button
          @click="minimize"
          @mousedown.stop
          class="inline-flex items-center justify-center w-11 h-8 hover:bg-accent/50 transition-colors"
        >
          <Minus :size="14" />
        </button>
        <button
          @click="toggleMaximize"
          @mousedown.stop
          class="inline-flex items-center justify-center w-11 h-8 hover:bg-accent/50 transition-colors"
        >
          <Copy v-if="isMaximized" :size="12" />
          <Square v-else :size="12" />
        </button>
        <button
          @click="close"
          @mousedown.stop
          class="inline-flex items-center justify-center w-11 h-8 hover:bg-destructive hover:text-destructive-foreground transition-colors"
        >
          <X :size="16" />
        </button>
      </div>
    </div>

    <!-- Title Container -->
    <div 
      class="flex-1 flex items-center h-full pointer-events-none px-4"
      :class="store.controlsSide === 'left' ? 'justify-start' : 'justify-end'"
    >
      <span class="text-xs font-medium text-muted-foreground opacity-50">Claket</span>
    </div>
  </div>
</template>

<style scoped>
/* Ensure the titlebar doesn't block clicks to things below it, except for the buttons */
div {
  transition: background-color 0.2s;
}
</style>
