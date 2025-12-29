<script setup lang="ts">
import { ref, computed } from "vue";
import { useAudioStore } from "@/stores/audio";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
  ContextMenuSeparator,
} from "@/components/ui/context-menu";
import { Slider } from "@/components/ui/slider";
import { open } from "@tauri-apps/plugin-dialog";
import { Play, Pause, Square, Pencil, FolderOpen, RefreshCcw, Trash2 } from "lucide-vue-next";
import GlowingEffect from "@/components/ui/GlowingEffect.vue";
import Input from "@/components/ui/Input.vue";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog'

const props = defineProps<{
  id: number;
}>();

const store = useAudioStore();
const button = computed(() => store.buttons.find(b => b.id === props.id));
const isRenameOpen = ref(false);
const newName = ref("");

const openRename = () => {
  newName.value = button.value?.name || "";
  isRenameOpen.value = true;
};

const handleRename = () => {
  if (newName.value.trim()) {
    store.updateButton(props.id, { name: newName.value.trim() });
  }
  isRenameOpen.value = false;
};

const handlePlay = async () => {
  if (!button.value?.path) {
    await handleChooseFile();
    return;
  }
  await store.playSound(props.id);
};

const handleStop = async (e?: Event) => {
  e?.preventDefault();
  e?.stopPropagation();
  await store.stopAllInstances(props.id);
};

const handleTogglePause = async (e?: Event) => {
  e?.preventDefault();
  e?.stopPropagation();
  await store.togglePauseLatest(props.id);
};

const handleDrop = (e: DragEvent) => {
  e.preventDefault();
  if (e.dataTransfer?.files && e.dataTransfer.files.length > 0) {
    const file = e.dataTransfer.files[0];
    const path = (file as any).path; 
    if (path) {
      store.updateButtonWithCopy(props.id, path, file.name.replace(/\.[^/.]+$/, ""));
    }
  }
};

const handleChooseFile = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Audio",
          extensions: ["mp3", "wav", "ogg", "flac", "m4a", "aac"]
        }
      ]
    });
    
    if (selected && typeof selected === 'string' && selected.length > 0) {
      store.updateButtonWithCopy(props.id, selected);
    }
  } catch (e) {
    console.error("Failed to open file dialog", e);
  }
};

const handleVolumeChange = (v: number[]) => {
  if (v && v.length > 0) {
    store.updateButton(props.id, { volume: v[0] });
  }
};

const reset = async () => {
  if (button.value?.path) {
    await store.deleteSoundFile(button.value.path);
  }
  
  store.updateButton(props.id, { 
    path: null, 
    name: `Button ${props.id + 1}`, 
    volume: 0.5,
    activeInstances: 0,
    isPaused: false
  });
};

const activeInstancesList = computed(() => {
  return store.queueList.filter(item => item.id === props.id.toString());
});

const progressPercent = computed(() => {
  if (activeInstancesList.value.length === 0) return 0;
  const latest = activeInstancesList.value[0]; 
  return (latest.position_ms / latest.duration_ms) * 100;
});
</script>

<template>
  <ContextMenu>
    <ContextMenuTrigger class="w-full h-full block">
      <div
        class="relative h-full rounded-xl border p-1"
        @click="handlePlay"
      >
        <GlowingEffect
          :spread="40"
          :glow="button?.activeInstances && button.activeInstances > 0"
          :disabled="false"
          :proximity="64"
          :inactive-zone="0.01"
        />
        
        <div
          class="relative flex flex-col items-center justify-center w-full aspect-square transition-all duration-300 group cursor-pointer select-none bg-card/60 backdrop-blur-sm border border-white/5 rounded-lg overflow-hidden"
          :class="[
            button?.activeInstances && button.activeInstances > 0 
              ? (button.isPaused ? 'bg-orange-500/10' : 'bg-primary/5 shadow-[0_0_20px_rgba(59,130,246,0.1)] scale-[1.01]') 
              : '',
          ]"
        >
          <!-- Controls Overlay -->
          <div v-if="button?.activeInstances && button.activeInstances > 0" class="absolute top-2 right-2 flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity z-20">
             <button @click.stop="handleTogglePause" class="p-1.5 hover:bg-accent rounded-full bg-background/80 shadow-sm border border-border">
                <Pause v-if="!button.isPaused" :size="14" />
                <Play v-else :size="14" />
             </button>
             <button @click.stop="handleStop" class="p-1.5 hover:bg-accent rounded-full bg-background/80 shadow-sm border border-border text-destructive">
                <Square :size="14" fill="currentColor" />
             </button>
          </div>

          <div class="w-full h-full flex flex-col items-center justify-center p-4">
            <div v-if="button?.path" class="text-center p-4 w-full">
              <div class="font-bold truncate w-full text-lg mb-1 group-hover:text-primary transition-colors text-slate-950 dark:text-white">
                {{ button.name }}
              </div>
              <div class="text-xs text-muted-foreground mt-1 flex flex-col items-center gap-1">
                 <div class="flex items-center gap-2 text-slate-700 dark:text-slate-300">
                    <span v-if="button.activeInstances > 0" class="text-[10px] font-mono bg-primary px-2 py-0.5 rounded text-primary-foreground animate-pulse">
                      {{ button.activeInstances }}x
                    </span>
                    <span class="font-medium">{{ Math.round(button.volume * 100) }}%</span>
                 </div>
              </div>
            </div>
            
            <div v-else class="text-muted-foreground text-sm flex flex-col items-center gap-3 opacity-40 group-hover:opacity-100 transition-opacity">
              <div class="w-10 h-10 rounded-full bg-muted flex items-center justify-center border border-white/10 shadow-inner">
                 <span class="text-2xl font-light">+</span>
              </div>
              <div class="text-[10px] font-medium tracking-widest uppercase">Drop Audio</div>
            </div>
          </div>

          <!-- Individual progress bar -->
          <div 
            v-if="button?.activeInstances && button.activeInstances > 0" 
            class="absolute bottom-0 left-0 h-1 bg-primary/20 w-full"
          >
            <div class="h-full bg-primary transition-[width] duration-150 ease-linear shadow-[0_0_10px_rgba(59,130,246,0.5)]" :style="{ width: `${progressPercent}%` }"></div>
          </div>
        </div>
      </div>
    </ContextMenuTrigger>
    <ContextMenuContent class="w-56">
      <ContextMenuItem @select="handleChooseFile" class="gap-2">
        <FolderOpen class="size-4 opacity-70" />
        Choose File...
      </ContextMenuItem>
      <ContextMenuItem @select="openRename" class="gap-2">
        <Pencil class="size-4 opacity-70" />
        Rename Sound...
      </ContextMenuItem>
      <ContextMenuItem v-if="button?.activeInstances && button.activeInstances > 0" @select="handleTogglePause" class="gap-2">
        <template v-if="button.isPaused">
          <Play class="size-4 opacity-70" />
          Resume Latest
        </template>
        <template v-else>
          <Pause class="size-4 opacity-70" />
          Pause Latest
        </template>
      </ContextMenuItem>
      <ContextMenuItem v-if="button?.activeInstances && button.activeInstances > 0" @select="handleStop" class="gap-2">
        <Square class="size-4 opacity-70" />
        Stop All instances
      </ContextMenuItem>
      <ContextMenuSeparator />
      <ContextMenuItem @select="reset" class="gap-2 text-destructive focus:text-destructive">
        <RefreshCcw class="size-4 opacity-70" />
        Reset Button
      </ContextMenuItem>
      <div class="p-2">
        <label class="text-[10px] mb-2 block font-bold uppercase tracking-wider opacity-50">Button Volume</label>
        <Slider 
          :model-value="[button?.volume || 0.5]"
          :max="1" 
          :step="0.01"
          @update:model-value="handleVolumeChange"
        />
      </div>
    </ContextMenuContent>
  </ContextMenu>

  <AlertDialog v-model:open="isRenameOpen">
    <AlertDialogContent>
      <AlertDialogHeader>
        <AlertDialogTitle>Rename Sound</AlertDialogTitle>
        <AlertDialogDescription>
          Enter a new name for this sound button.
        </AlertDialogDescription>
      </AlertDialogHeader>
      <div class="py-4">
        <Input v-model="newName" placeholder="Sound name" @keyup.enter="handleRename" />
      </div>
      <AlertDialogFooter>
        <AlertDialogCancel>Cancel</AlertDialogCancel>
        <AlertDialogAction @click="handleRename">Rename</AlertDialogAction>
      </AlertDialogFooter>
    </AlertDialogContent>
  </AlertDialog>
</template>
