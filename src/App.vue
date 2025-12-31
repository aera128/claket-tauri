<script setup lang="ts">
import { onMounted, computed, ref, watch } from "vue";
import { useAudioStore, ACCENT_COLORS } from "./stores/audio";
import DeviceSelector from "./components/DeviceSelector.vue";
import GlowButton from "./components/GlowButton.vue";
import TitleBar from "./components/TitleBar.vue";
import { Slider } from "@/components/ui/slider";
import { Toaster } from "@/components/ui/sonner";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Square, Music, Play, Pause, ListMusic, ChevronDown, Trash2, Moon, Sun, ChevronLeft, ChevronRight, Plus, Pencil, Settings, Monitor, Laptop, AlignLeft, AlignRight } from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import { Switch } from "@/components/ui/switch";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
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
  AlertDialogTrigger,
} from '@/components/ui/alert-dialog'
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import AuroraBackground from "@/components/ui/AuroraBackground.vue";
import { useDark, useToggle } from "@vueuse/core";
import logo from "@/assets/logo.png";

const store = useAudioStore();
const isDark = useDark();
const toggleDark = useToggle(isDark);

watch(isDark, (val) => {
  store.applyAccentColor(val);
});

const gridRef = ref<HTMLElement | null>(null);

const isPageRenameOpen = ref(false);
const newPageName = ref("");

const openPageRename = () => {
  newPageName.value = store.pageNames[store.currentPage] || `Page ${store.currentPage + 1}`;
  isPageRenameOpen.value = true;
};

const handlePageRename = () => {
  if (newPageName.value.trim()) {
    store.updatePageName(store.currentPage, newPageName.value.trim());
  }
  isPageRenameOpen.value = false;
};

onMounted(() => {
  store.init();

  listen("file-dropped", (event: any) => {
    const { path, x, y } = event.payload;
    if (gridRef.value) {
      const rect = gridRef.value.getBoundingClientRect();
      const relativeX = x - rect.left;
      const relativeY = y - rect.top;

      if (relativeX >= 0 && relativeX <= rect.width && relativeY >= 0 && relativeY <= rect.height) {
        const colWidth = rect.width / 4;
        const rowHeight = rect.height / Math.ceil(store.itemsPerPage / 4);
        const col = Math.floor(relativeX / colWidth);
        const row = Math.floor(relativeY / rowHeight);
        
        store.updateButtonByCoords(store.currentPage, col, row, path);
      }
    }
  });
});

const handleMasterVolume = (v: number[]) => {
  if (v && v.length > 0) {
    store.setMasterVolume(v[0]);
  }
};

const localSeekValue = ref<number | null>(null);
const wasPlayingBeforeSeek = ref<Map<number, boolean>>(new Map());

const handleSeek = (instanceId: number, v: number[]) => {
  if (v && v.length > 0 && store.seekingInstanceId === instanceId) {
    localSeekValue.value = v[0];
  }
};

const handleSeekStart = async (instanceId: number) => {
  const item = store.activeProgresses.get(instanceId);
  if (item) {
    store.seekingInstanceId = instanceId;
    localSeekValue.value = item.position_ms;
    
    const wasPlaying = !item.is_paused;
    wasPlayingBeforeSeek.value.set(instanceId, wasPlaying);

    if (wasPlaying) {
      await store.togglePauseInstance(instanceId);
    }
  }
};

const handleSeekEnd = async (instanceId: number) => {
  const item = store.activeProgresses.get(instanceId);
  const wasPlaying = wasPlayingBeforeSeek.value.get(instanceId);

  if (item && localSeekValue.value !== null) {
    const finalPos = localSeekValue.value;
    
    store.seekRecovery.set(instanceId, finalPos);
    
    await invoke("seek_instance", { instanceId, positionMs: Math.round(finalPos) });
    item.position_ms = finalPos;
  }
  
  if (wasPlaying && item && item.is_paused) {
    await store.togglePauseInstance(instanceId);
  }
  
  wasPlayingBeforeSeek.value.delete(instanceId);
  store.seekingInstanceId = null;
  localSeekValue.value = null;
};

const accentColors = Object.entries(ACCENT_COLORS).map(([name, values]) => ({
  name,
  ...values
}));

const getVUColor = (index: number, total: number) => {
  const position = index / total;
  // Red: Top 10% (0.9 to 1.0)
  // Yellow: 70% to 90% (0.7 to 0.9)
  // Green: 0% to 70%
  if (position > 0.90) return 'text-red-500 bg-red-500';      
  if (position > 0.70) return 'text-yellow-400 bg-yellow-400'; 
  return 'text-emerald-500 bg-emerald-500';                 
};

const isSegmentActive = (index: number, total: number) => {
  const peak = store.masterLevels.peak;
  if (peak <= 0) return false;
  
  // Power scale (0.5) to ensure signal actually reaches the top segments
  const normalized = Math.pow(peak, 0.5);
  const threshold = (index - 1) / total;
  return normalized > threshold;
};
</script>

<template>
  <AuroraBackground className="!h-auto !block min-h-screen">
    <TitleBar />
    <div class="w-full h-screen flex flex-col relative overflow-hidden">
      <header class="w-full p-6 flex justify-center fixed top-2 left-0 right-0 z-50 pointer-events-none">
        <div class="island-navbar pointer-events-auto flex items-center justify-between gap-6 px-6 h-12 rounded-full border border-border bg-card/60 shadow-2xl min-w-[500px] max-w-[95%] transition-all duration-500 hover:scale-[1.01] hover:border-primary/20 text-slate-950 dark:text-white"
             :class="{ 'backdrop-blur-2xl': !store.reducedMotion }">
          <h1 class="text-base font-bold tracking-tight flex items-center gap-2 text-nowrap">
            <img :src="logo" class="w-6 h-6 object-contain brightness-110" alt="Claket Logo" />
            Claket
          </h1>
          
          <div class="h-5 w-[1px] bg-border/50"></div>
          
          <div class="flex items-center gap-4 flex-1">
            <DeviceSelector class="scale-90 flex-1" />
          </div>

          <div class="h-5 w-[1px] bg-border/50"></div>

          <div class="flex items-center gap-1">
            <Dialog>
              <DialogTrigger as-child>
                <Button variant="ghost" size="icon" class="rounded-full h-8 w-8 shrink-0">
                  <Settings :size="18" />
                </Button>
              </DialogTrigger>
              <DialogContent class="sm:max-w-[425px]">
                <DialogHeader>
                  <DialogTitle>Settings</DialogTitle>
                </DialogHeader>
                <div class="grid gap-6 py-4">
                  <div class="flex items-center justify-between">
                    <div class="space-y-0.5">
                      <div class="text-sm font-medium">Reduced Motion</div>
                      <div class="text-xs text-muted-foreground">Disable background animations</div>
                    </div>
                    <Switch 
                      :model-value="store.reducedMotion" 
                      @update:model-value="store.setReducedMotion" 
                    />
                  </div>
                  <div class="space-y-3">
                    <div class="text-sm font-medium">Layout Style</div>
                    <div class="grid grid-cols-2 gap-2">
                      <Button 
                        variant="outline" 
                        class="justify-start gap-2 h-12"
                        :class="{ 'border-primary bg-primary/5': store.titlebarStyle === 'windows' }"
                        @click="store.setTitlebarStyle('windows')"
                      >
                        <Monitor :size="16" />
                        <div class="text-left">
                          <div class="text-xs font-bold">Windows</div>
                          <div class="text-[10px] text-muted-foreground">Controls on right</div>
                        </div>
                      </Button>
                      <Button 
                        variant="outline" 
                        class="justify-start gap-2 h-12"
                        :class="{ 'border-primary bg-primary/5': store.titlebarStyle === 'mac' }"
                        @click="store.setTitlebarStyle('mac')"
                      >
                        <Laptop :size="16" />
                        <div class="text-left">
                          <div class="text-xs font-bold">macOS</div>
                          <div class="text-[10px] text-muted-foreground">Controls on left</div>
                        </div>
                      </Button>
                    </div>
                  </div>
                  <div class="space-y-3">
                    <div class="text-sm font-medium">Controls Position</div>
                    <div class="grid grid-cols-2 gap-2">
                      <Button 
                        variant="outline" 
                        class="justify-start gap-2 h-12"
                        :class="{ 'border-primary bg-primary/5': store.controlsSide === 'left' }"
                        @click="store.setControlsSide('left')"
                      >
                        <AlignLeft :size="16" />
                        <div class="text-left">
                          <div class="text-xs font-bold">Left Side</div>
                        </div>
                      </Button>
                      <Button 
                        variant="outline" 
                        class="justify-start gap-2 h-12"
                        :class="{ 'border-primary bg-primary/5': store.controlsSide === 'right' }"
                        @click="store.setControlsSide('right')"
                      >
                        <AlignRight :size="16" />
                        <div class="text-left">
                          <div class="text-xs font-bold">Right Side</div>
                        </div>
                      </Button>
                    </div>
                  </div>
                  <div class="space-y-3">
                    <div class="text-sm font-medium">Accent Color</div>
                    <div class="grid grid-cols-6 gap-2">
                      <button
                        v-for="color in accentColors"
                        :key="color.name"
                        @click="store.setAccentColor(color.name, isDark)"
                        class="w-10 h-10 rounded-full border-2 transition-all hover:scale-110 flex items-center justify-center overflow-hidden p-0"
                        :class="store.accentColor === color.name ? 'border-foreground ring-2 ring-primary/20' : 'border-transparent'"
                        :title="color.name"
                      >
                        <div class="w-full h-full flex flex-col">
                          <div class="flex-1 w-full" :style="{ backgroundColor: color.light }"></div>
                          <div class="flex-1 w-full" :style="{ backgroundColor: color.dark }"></div>
                        </div>
                      </button>
                    </div>
                  </div>
                </div>
              </DialogContent>
            </Dialog>

            <Button variant="ghost" size="icon" class="rounded-full h-8 w-8 shrink-0" @click="toggleDark()">
               <Moon v-if="isDark" :size="18" />
               <Sun v-else :size="18" />
            </Button>
          </div>
        </div>
      </header>
      
      <ScrollArea class="flex-1 w-full transition-all duration-500 h-[1px]" :class="{ 'blur-md scale-95 opacity-50': store.isQueueExpanded }">
        <div class="max-w-4xl mx-auto flex flex-col gap-6 p-6 pt-24 pb-32">
          <div class="flex items-center justify-between px-2">
            <h2 class="text-xl font-bold text-slate-900 dark:text-slate-100 flex items-center gap-2 group cursor-pointer" @click="openPageRename">
              <span class="w-2 h-2 rounded-full bg-primary animate-pulse"></span>
              {{ store.pageNames[store.currentPage] || `Page ${store.currentPage + 1}` }}
              <Pencil :size="14" class="opacity-0 group-hover:opacity-50 transition-opacity" />
            </h2>
            <div class="flex items-center gap-2 bg-card/40 p-1 rounded-full border border-border">
              <Button variant="ghost" size="icon" class="rounded-full h-8 w-8" @click="store.prevPage" :disabled="store.currentPage === 0">
                <ChevronLeft :size="16" />
              </Button>
              <div class="flex items-center gap-1 px-2">
                <button 
                  v-for="p in store.totalPages" 
                  :key="p"
                  @click="store.setPage(p-1)"
                  class="w-2 h-2 rounded-full transition-all duration-300"
                  :class="store.currentPage === p-1 ? 'bg-primary w-6' : 'bg-muted-foreground/30 hover:bg-muted-foreground/50'"
                ></button>
              </div>
              <Button variant="ghost" size="icon" class="rounded-full h-8 w-8" @click="store.nextPage" :disabled="store.currentPage === store.totalPages - 1">
                <ChevronRight :size="16" />
              </Button>
              <div class="h-4 w-[1px] bg-border mx-1"></div>
              
              <AlertDialog>
                <AlertDialogTrigger as-child>
                  <Button variant="ghost" size="icon" class="rounded-full h-8 w-8 hover:bg-destructive/10 hover:text-destructive transition-colors" title="Delete Page">
                    <Trash2 :size="16" />
                  </Button>
                </AlertDialogTrigger>
                <AlertDialogContent>
                  <AlertDialogHeader>
                    <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
                    <AlertDialogDescription>
                      This will permanently delete the current page and all its configured sound buttons. This action cannot be undone.
                    </AlertDialogDescription>
                  </AlertDialogHeader>
                  <AlertDialogFooter>
                    <AlertDialogCancel>Cancel</AlertDialogCancel>
                    <AlertDialogAction @click="store.removeCurrentPage" class="bg-destructive text-destructive-foreground hover:bg-destructive/90">
                      Delete Page
                    </AlertDialogAction>
                  </AlertDialogFooter>
                </AlertDialogContent>
              </AlertDialog>

              <Button variant="ghost" size="icon" class="rounded-full h-8 w-8 hover:bg-primary/10 hover:text-primary transition-colors" @click="store.addPage" title="Add Page">
                <Plus :size="16" />
              </Button>
            </div>
          </div>

          <div ref="gridRef" class="grid grid-cols-2 md:grid-cols-4 gap-4">
            <GlowButton 
              v-for="btn in store.paginatedButtons" 
              :key="btn.id" 
              :id="btn.id" 
            />
          </div>
        </div>
      </ScrollArea>

      <Transition name="slide-up">
        <div v-if="store.isQueueExpanded" class="fixed inset-0 z-30 bg-background/60 flex flex-col pt-20"
             :class="{ 'backdrop-blur-xl': !store.reducedMotion }">
          <div class="flex-1 overflow-y-auto p-8 max-w-4xl mx-auto w-full text-slate-950 dark:text-white">
             <div class="flex items-center justify-between mb-8">
                <h2 class="text-3xl font-bold flex items-center gap-3">
                   <ListMusic :size="32" class="text-primary" />
                   Active Sounds
                </h2>
                <Button variant="ghost" size="icon" @click="store.isQueueExpanded = false">
                   <ChevronDown :size="32" />
                </Button>
             </div>

             <div v-if="store.queueList.length === 0" class="flex flex-col items-center justify-center h-64 text-muted-foreground gap-4">
                <Music :size="48" class="opacity-20" />
                <p>No sounds are currently playing</p>
             </div>

             <div class="space-y-4">
                <div v-for="item in store.queueList" :key="item.instance_id" 
                     class="bg-card border rounded-xl p-4 flex flex-col gap-3 group animate-in slide-in-from-bottom-2 duration-300">
                  <div class="flex items-center justify-between">
                    <div class="flex items-center gap-4">
                      <div class="w-10 h-10 bg-primary/10 rounded flex items-center justify-center text-primary">
                        <Music :size="20" :class="{ 'animate-pulse': !item.is_paused }" />
                      </div>
                      <div>
                        <div class="font-bold text-lg leading-tight">{{ item.name }}</div>
                        <div class="text-xs text-muted-foreground font-mono uppercase tracking-widest text-nowrap text-[10px]">
                          ID: {{ item.instance_id }} • {{ Math.floor(item.position_ms / 1000) }}s / {{ Math.floor(item.duration_ms / 1000) }}s
                        </div>
                      </div>
                    </div>
                    <div class="flex items-center gap-2 shrink-0">
                      <Button variant="outline" size="icon" class="rounded-full" @click="store.togglePauseInstance(item.instance_id)">
                        <Pause v-if="!item.is_paused" :size="18" fill="currentColor" />
                        <Play v-else :size="18" fill="currentColor" />
                      </Button>
                      <Button variant="destructive" size="icon" class="rounded-full" @click="store.stopInstance(item.instance_id)">
                        <Square :size="16" fill="currentColor" />
                      </Button>
                    </div>
                  </div>
                  
                  <div class="flex items-center gap-4 px-2">
                    <Slider 
                      :model-value="store.seekingInstanceId === item.instance_id && localSeekValue !== null ? [localSeekValue] : [item.position_ms]" 
                      :max="item.duration_ms" 
                      :step="1" 
                      @update:modelValue="(v) => handleSeek(item.instance_id, v)"
                      @pointerdown.capture="handleSeekStart(item.instance_id)"
                      @pointerup="handleSeekEnd(item.instance_id)"
                    />
                  </div>
                </div>
             </div>
          </div>
        </div>
      </Transition>
      
      <footer class="fixed bottom-0 left-0 right-0 border-t bg-card/80 z-40 text-slate-950 dark:text-white"
              :class="{ 'backdrop-blur-xl': !store.reducedMotion }">
        <div class="w-full h-1 bg-accent/30 relative cursor-pointer group">
          <Slider 
            v-if="store.latestProgress"
            class="absolute -top-2 left-0 w-full h-5 opacity-0 group-hover:opacity-100 transition-opacity z-10"
            :model-value="store.seekingInstanceId === store.latestProgress.instance_id && localSeekValue !== null ? [localSeekValue] : [store.latestProgress.position_ms]" 
            :max="store.latestProgress.duration_ms" 
            :step="1" 
            @update:modelValue="(v) => handleSeek(store.latestProgress!.instance_id, v)"
            @pointerdown.capture="handleSeekStart(store.latestProgress!.instance_id)"
            @pointerup="handleSeekEnd(store.latestProgress!.instance_id)"
          />
          <div 
            v-if="store.latestProgress"
            class="h-full bg-primary relative transition-[width] duration-150 ease-linear pointer-events-none"
            :style="{ width: `${((store.seekingInstanceId === store.latestProgress.instance_id && localSeekValue !== null ? localSeekValue : store.latestProgress.position_ms) / store.latestProgress.duration_ms) * 100}%` }"
          >
          </div>
        </div>

        <div class="max-w-[1400px] mx-auto px-6 h-20 flex items-center justify-between gap-4">
          <div class="flex items-center gap-4 w-[30%] min-w-0">
            <div class="w-12 h-12 bg-accent/50 rounded-md flex items-center justify-center flex-shrink-0 overflow-hidden relative group cursor-pointer"
                 @click="store.isQueueExpanded = !store.isQueueExpanded">
              <Music v-if="!store.latestProgress" class="text-muted-foreground" />
              <div v-else class="w-full h-full bg-primary/20 flex items-center justify-center">
                 <Music :size="20" class="text-primary" :class="{ 'animate-pulse': !store.latestProgress.is_paused }" />
              </div>
              <div class="absolute inset-0 bg-black/40 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity text-white">
                 <ChevronDown v-if="store.isQueueExpanded" :size="20" />
                 <ListMusic v-else :size="20" />
              </div>
            </div>
            <div class="min-w-0 flex-1">
              <div class="text-sm font-semibold truncate cursor-pointer hover:underline text-slate-950 dark:text-white" @click="store.isQueueExpanded = !store.isQueueExpanded">
                 {{ store.latestProgress?.name || 'No sound playing' }}
              </div>
              <div class="text-[10px] text-muted-foreground uppercase tracking-wider font-bold">
                 {{ store.latestProgress ? `${Math.floor(store.latestProgress.position_ms / 1000)}s / ${Math.floor(store.latestProgress.duration_ms / 1000)}s` : 'Ready' }}
              </div>
            </div>
          </div>

          <div class="flex flex-col items-center gap-1 flex-1">
            <div class="flex items-center gap-4">
              <Button 
                variant="ghost" 
                size="icon" 
                class="hover:text-primary transition-colors h-8 w-8 rounded-full"
                @click="store.stopAll"
                title="Stop All"
              >
                <Trash2 :size="18" />
              </Button>
              
              <Button 
                variant="default" 
                size="icon" 
                class="h-12 w-12 rounded-full shadow-md hover:scale-105 active:scale-95 transition-all duration-300"
                @click="store.latestProgress && store.togglePauseInstance(store.latestProgress.instance_id)"
                :disabled="!store.latestProgress"
              >
                <Pause v-if="store.latestProgress && !store.latestProgress.is_paused" :size="24" fill="currentColor" />
                <Play v-else :size="24" fill="currentColor" />
              </Button>

              <Button 
                variant="ghost" 
                size="icon" 
                class="hover:text-destructive transition-colors h-8 w-8 rounded-full"
                @click="store.latestProgress && store.stopInstance(store.latestProgress.instance_id)"
                :disabled="!store.latestProgress"
                title="Stop current"
              >
                <Square :size="18" fill="currentColor" />
              </Button>
            </div>
          </div>

          <div class="flex items-center gap-4 w-[30%] justify-end relative h-full">
            <div class="relative flex items-center h-full">
              <div 
                class="flex flex-col-reverse gap-[1px] px-[1px] py-[2px] bg-black/20 dark:bg-black/40 rounded-sm border border-border/30 cursor-pointer transition-all duration-300 hover:scale-105 active:scale-95 group relative z-50"
                :class="store.isVUMeterExpanded ? 'h-56 w-8 -translate-y-24 shadow-2xl bg-card/95 backdrop-blur-md ring-1 ring-white/10' : 'h-10 w-4'"
                @click="store.isVUMeterExpanded = !store.isVUMeterExpanded"
                title="Click to expand VU Meter"
              >
                <div v-for="i in (store.isVUMeterExpanded ? 48 : 12)" :key="i" 
                     class="w-full rounded-[1px] transition-all duration-75 flex-1"
                     :class="getVUColor(i, store.isVUMeterExpanded ? 48 : 12)"
                     :style="{ 
                       opacity: isSegmentActive(i, store.isVUMeterExpanded ? 48 : 12) ? 1 : 0.1,
                       boxShadow: !store.reducedMotion && isSegmentActive(i, store.isVUMeterExpanded ? 48 : 12) ? '0 0 6px currentColor' : 'none'
                     }">
                </div>
                
                <!-- Scale labels when expanded -->
                <div v-if="store.isVUMeterExpanded" class="absolute -left-7 inset-y-0 flex flex-col justify-between text-[7px] font-mono text-muted-foreground py-1 pointer-events-none items-end pr-1">
                  <span class="text-red-500 font-bold">0</span>
                  <span>-3</span>
                  <span>-6</span>
                  <span>-12</span>
                  <span>-18</span>
                  <span>-24</span>
                  <span>-36</span>
                  <span>-48</span>
                </div>
              </div>
            </div>

            <Button variant="ghost" size="icon" class="rounded-full" @click="store.isQueueExpanded = !store.isQueueExpanded" :class="{ 'text-primary bg-primary/10': store.isQueueExpanded }">
               <ListMusic :size="18" />
            </Button>

            <div class="flex items-center gap-2 group/vol min-w-[150px]">
              <Music :size="16" class="text-muted-foreground" />
              <div class="w-24">
                <Slider 
                  :model-value="[store.masterVolume]" 
                  :max="1" 
                  :step="0.01" 
                  @update:model-value="handleMasterVolume"
                />
              </div>
              <span class="text-[10px] font-mono w-8 text-right text-slate-900 dark:text-slate-100">{{ Math.round(store.masterVolume * 100) }}%</span>
            </div>
          </div>
        </div>
      </footer>
      <Toaster />
    </div>
  </AuroraBackground>

  <AlertDialog v-model:open="isPageRenameOpen">
    <AlertDialogContent>
      <AlertDialogHeader>
        <AlertDialogTitle>Rename Page</AlertDialogTitle>
        <AlertDialogDescription>
          Enter a new name for this page.
        </AlertDialogDescription>
      </AlertDialogHeader>
      <div class="py-4">
        <Input v-model="newPageName" placeholder="Page name" @keyup.enter="handlePageRename" />
      </div>
      <AlertDialogFooter>
        <AlertDialogCancel>Cancel</AlertDialogCancel>
        <AlertDialogAction @click="handlePageRename">Rename</AlertDialogAction>
      </AlertDialogFooter>
    </AlertDialogContent>
  </AlertDialog>
</template>

<style>
:root {
  font-family: Inter, system-ui, Avenir, Helvetica, Arial, sans-serif;
}
body {
  margin: 0;
  padding: 0;
}

.slide-up-enter-active,
.slide-up-leave-active {
  transition: transform 0.5s cubic-bezier(0.32, 0.72, 0, 1);
}

.slide-up-enter-from,
.slide-up-leave-to {
  transform: translateY(100%);
}
</style>
