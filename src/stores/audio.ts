import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { Store } from "@tauri-apps/plugin-store";
import { listen } from "@tauri-apps/api/event";
import { toast } from "vue-sonner";

export interface SoundButton {
  id: number;
  path: string | null;
  name: string;
  volume: number;
  color: string;
  isPaused: boolean;
  activeInstances: number;
}

export interface AudioProgress {
  id: string;
  instance_id: number;
  name: string;
  position_ms: number;
  duration_ms: number;
  is_paused: boolean;
  last_sync_time: number;
  last_sync_pos: number;
}

export const ACCENT_COLORS: Record<string, { light: string, dark: string }> = {
  'Default': { light: 'oklch(0.21 0.006 285.885)', dark: 'oklch(0.985 0 0)' },
  'Indigo': { light: 'oklch(0.55 0.22 260)', dark: 'oklch(0.65 0.18 265)' },
  'Violet': { light: 'oklch(0.60 0.25 292)', dark: 'oklch(0.75 0.20 290)' },
  'Fuchsia': { light: 'oklch(0.60 0.25 330)', dark: 'oklch(0.75 0.20 330)' },
  'Rose': { light: 'oklch(0.627 0.265 303.9)', dark: 'oklch(0.70 0.20 300)' },
  'Crimson': { light: 'oklch(0.577 0.245 27.325)', dark: 'oklch(0.65 0.20 30)' },
  'Orange': { light: 'oklch(0.70 0.20 45)', dark: 'oklch(0.75 0.18 50)' },
  'Amber': { light: 'oklch(0.769 0.188 70.08)', dark: 'oklch(0.80 0.15 75)' },
  'Lime': { light: 'oklch(0.80 0.22 135)', dark: 'oklch(0.85 0.18 140)' },
  'Emerald': { light: 'oklch(0.627 0.265 149.21)', dark: 'oklch(0.75 0.15 150)' },
  'Cyan': { light: 'oklch(0.72 0.14 210)', dark: 'oklch(0.80 0.12 210)' },
  'Sky': { light: 'oklch(0.596 0.145 252.69)', dark: 'oklch(0.70 0.14 250)' },
};

export const useAudioStore = defineStore("audio", {
  state: () => ({
    buttons: [] as SoundButton[],
    masterVolume: 1.0,
    devices: [] as string[],
    currentDevice: "Default",
    isLoaded: false,
    activeProgresses: new Map<number, AudioProgress>(),
    isQueueExpanded: false,
    seekingInstanceId: null as number | null,
    seekRecovery: new Map<number, number>(),
    currentPage: 0,
    itemsPerPage: 16,
    totalPages: 4,
    pageNames: [] as string[],
    dragPosition: null as { page: number, x: number, y: number } | null,
    lastUpdateTimestamp: new Map<number, number>(),
    reducedMotion: false,
    accentColor: "Default",
    masterLevels: { peak: 0, rms: 0 },
    isVUMeterExpanded: false,
    titlebarStyle: "windows" as "windows" | "mac",
    controlsSide: "right" as "left" | "right",
  }),
  getters: {
    queueList: (state) => Array.from(state.activeProgresses.values()).reverse(),
    latestProgress: (state) => {
        const list = Array.from(state.activeProgresses.values());
        if (list.length === 0) return null;
        return list[list.length - 1];
    },
    paginatedButtons: (state) => {
      const start = state.currentPage * state.itemsPerPage;
      return state.buttons.slice(start, start + state.itemsPerPage);
    }
  },
  actions: {
    async init() {
      if (this.buttons.length === 0) {
        const totalButtons = this.itemsPerPage * this.totalPages;
        for (let i = 0; i < totalButtons; i++) {
          this.buttons.push({
            id: i,
            path: null,
            name: `Button ${i + 1}`,
            volume: 0.5,
            color: "bg-secondary",
            isPaused: false,
            activeInstances: 0,
          });
        }
      }

      if (this.pageNames.length === 0) {
        for (let i = 0; i < this.totalPages; i++) {
          this.pageNames.push(`Page ${i + 1}`);
        }
      }
      
      await this.loadDevices();
      await this.loadSettings();
      this.setupListeners();
      this.isLoaded = true;
      this.preloadCurrentPage();
      this.applyAccentColor();
      
      this.startAnimationLoop();
    },

    startAnimationLoop() {
      const loop = () => {
        const now = Date.now();
        this.activeProgresses.forEach((progress, instanceId) => {
          if (this.seekingInstanceId === instanceId) return;

          if (!progress.is_paused) {
             const elapsedSinceSync = now - progress.last_sync_time;
             const projected = progress.last_sync_pos + elapsedSinceSync;
             progress.position_ms = Math.min(progress.duration_ms, projected);
          }
        });
        requestAnimationFrame(loop);
      };
      requestAnimationFrame(loop);
    },

    setupListeners() {
      listen<{ peak: number, rms: number }>("master-level", (event) => {
        this.masterLevels = event.payload;
      });

      listen<AudioProgress>("audio-progress", (event) => {
        const payload = event.payload;
        
        if (this.seekingInstanceId === payload.instance_id) return;
        
        const recoveryTarget = this.seekRecovery.get(payload.instance_id);
        if (recoveryTarget !== undefined) {
            if (payload.position_ms >= recoveryTarget - 100) {
                this.seekRecovery.delete(payload.instance_id);
            } else {
                return;
            }
        }
        
        const existing = this.activeProgresses.get(payload.instance_id);
        const now = Date.now();

        if (existing) {
            existing.last_sync_pos = payload.position_ms;
            existing.last_sync_time = now;
            existing.is_paused = payload.is_paused;
            existing.duration_ms = payload.duration_ms;

            if (payload.is_paused) {
                existing.position_ms = payload.position_ms;
                if (Array.from(this.activeProgresses.values()).every(p => p.is_paused)) {
                  this.masterLevels = { peak: 0, rms: 0 };
                }
            }
        } else {
            this.activeProgresses.set(payload.instance_id, {
                ...payload,
                last_sync_time: now,
                last_sync_pos: payload.position_ms
            });
        }
        
        const button = this.buttons.find(b => b.id.toString() === payload.id);
        if (button) {
          button.isPaused = payload.is_paused;
        }
      });

      listen<number>("audio-finished", (event) => {
        const instanceId = event.payload;
        const progress = this.activeProgresses.get(instanceId);
        if (progress) {
            const button = this.buttons.find(b => b.id.toString() === progress.id);
            if (button && button.activeInstances > 0) {
              button.activeInstances--;
            }
            this.activeProgresses.delete(instanceId);
        }
      });
    },

    async loadDevices() {
      try {
        this.devices = await invoke("list_audio_devices");
      } catch (e) {
        console.error("Failed to list audio devices", e);
      }
    },

    async setOutputDevice(deviceName: string) {
      try {
        await invoke("set_audio_device", { deviceName });
        this.currentDevice = deviceName;
        await this.saveSettings();
        toast.success(`Output set to ${deviceName}`);
      } catch (e) {
        console.error("Failed to set audio device", e);
      }
    },

    async setMasterVolume(volume: number) {
      this.masterVolume = volume;
      try {
        await invoke("update_master_volume", { volume });
        await this.saveSettings();
      } catch (e) {
        console.error("Failed to update master volume", e);
      }
    },

    async playSound(buttonId: number) {
      const button = this.buttons.find(b => b.id === buttonId);
      if (!button || !button.path) return;

      try {
        button.activeInstances++;
        button.isPaused = false;
        
        await invoke("play_sound", { 
          id: buttonId.toString(), 
          path: button.path, 
          name: button.name,
          volume: button.volume 
        });
      } catch (e) {
        console.error("Failed to play sound", e);
        button.activeInstances--;
      }
    },

    async togglePauseInstance(instanceId: number) {
      try {
        const isPaused = await invoke<boolean>("toggle_pause_instance", { instanceId });
        const progress = this.activeProgresses.get(instanceId);
        if (progress) progress.is_paused = isPaused;
      } catch (e) {
        console.error("Failed to toggle pause", e);
      }
    },

    async stopInstance(instanceId: number) {
      try {
        await invoke("stop_instance", { instanceId });
        const progress = this.activeProgresses.get(instanceId);
        if (progress) {
            const button = this.buttons.find(b => b.id.toString() === progress.id);
            if (button && button.activeInstances > 0) button.activeInstances--;
            this.activeProgresses.delete(instanceId);
        }
      } catch (e) {
        console.error("Failed to stop instance", e);
      }
    },

    async togglePauseLatest(buttonId: number) {
      const activeForButton = this.queueList.filter(item => item.id === buttonId.toString());
      if (activeForButton.length > 0) {
        const latest = activeForButton[0];
        if (latest) {
          await this.togglePauseInstance(latest.instance_id);
        }
      }
    },

    async stopAllInstances(buttonId: number) {
      const activeForButton = this.queueList.filter(item => item.id === buttonId.toString());
      for (const instance of activeForButton) {
        await this.stopInstance(instance.instance_id);
      }
    },

    async stopAll() {
      try {
        await invoke("stop_all");
        this.buttons.forEach(b => {
          b.activeInstances = 0;
          b.isPaused = false;
        });
        this.activeProgresses.clear();
        toast.success("Stopped all sounds");
      } catch (e) {
        console.error("Failed to stop all", e);
      }
    },
    
    setPage(page: number) {
      if (page >= 0 && page < this.totalPages) {
        this.currentPage = page;
        this.preloadCurrentPage();
      }
    },

    nextPage() {
      if (this.currentPage < this.totalPages - 1) {
        this.currentPage++;
        this.preloadCurrentPage();
      }
    },

    prevPage() {
      if (this.currentPage > 0) {
        this.currentPage--;
        this.preloadCurrentPage();
      }
    },

    preloadCurrentPage() {
      this.paginatedButtons.forEach(btn => {
        if (btn.path) {
          invoke("preload_sound", { path: btn.path }).catch(() => {});
        }
      });
    },

    addPage() {
      const newPageCount = this.totalPages + 1;
      const startId = this.buttons.length;
      
      for (let i = 0; i < this.itemsPerPage; i++) {
        this.buttons.push({
          id: startId + i,
          path: null,
          name: `Button ${startId + i + 1}`,
          volume: 0.5,
          color: "bg-secondary",
          isPaused: false,
          activeInstances: 0,
        });
      }
      
      this.pageNames.push(`Page ${newPageCount}`);
      this.totalPages = newPageCount;
      this.currentPage = newPageCount - 1;
      this.saveSettings();
      toast.success(`Page ${newPageCount} added`);
    },

    async removeCurrentPage() {
      if (this.totalPages <= 1) {
        toast.error("Cannot delete the last page");
        return;
      }

      const startIndex = this.currentPage * this.itemsPerPage;
      
      const buttonsToRemove = this.buttons.slice(startIndex, startIndex + this.itemsPerPage);
      
      for (const btn of buttonsToRemove) {
        await this.stopAllInstances(btn.id);
        
        if (btn.path) {
          invoke("delete_sound_file", { path: btn.path }).catch(() => {
            console.warn("Failed to delete sound file:", btn.path);
          });
        }
      }

      this.buttons.splice(startIndex, this.itemsPerPage);
      this.pageNames.splice(this.currentPage, 1);
      
      this.buttons.forEach((btn, index) => {
        btn.id = index;
      });

      this.totalPages--;
      if (this.currentPage >= this.totalPages) {
        this.currentPage = Math.max(0, this.totalPages - 1);
      }

      this.saveSettings();
      toast.success(`Page deleted`);
    },

    setReducedMotion(enabled: boolean) {
      this.reducedMotion = enabled;
      this.saveSettings();
    },

    setAccentColor(colorName: string, isDark: boolean) {
      this.accentColor = colorName;
      this.saveSettings();
      this.applyAccentColor(isDark);
    },

    setTitlebarStyle(style: "windows" | "mac") {
      this.titlebarStyle = style;
      this.saveSettings();
    },

    setControlsSide(side: "left" | "right") {
      this.controlsSide = side;
      this.saveSettings();
    },

    applyAccentColor(isDark?: boolean) {
      const theme = isDark ?? document.documentElement.classList.contains('dark');
      const selected = ACCENT_COLORS[this.accentColor] || ACCENT_COLORS['Default']!;
      const finalColor = theme ? selected.dark : selected.light;
      document.documentElement.style.setProperty('--primary', finalColor);
    },

    updatePageName(index: number, name: string) {
      if (index >= 0 && index < this.pageNames.length) {
        this.pageNames[index] = name;
        this.saveSettings();
      }
    },

    async deleteSoundFile(path: string) {
      try {
        await invoke("delete_sound_file", { path });
      } catch (e) {
        console.warn("Failed to delete sound file:", path, e);
      }
    },
    
    updateButton(id: number, updates: Partial<SoundButton>) {
      const index = this.buttons.findIndex(b => b.id === id);
      if (index !== -1) {
        const current = this.buttons[index];
        if (current) {
          this.buttons[index] = {
            ...current,
            ...updates
          };
          this.saveSettings();
        }
      }
    },

    async updateButtonWithCopy(id: number, path: string, originalName?: string) {
      try {
        const localPath = await invoke<string>("save_sound_file", { path });
        const name = originalName || localPath.split('\\').pop()?.split('/').pop()?.replace(/\.[^/.]+$/, "") || "Audio";
        
        this.updateButton(id, {
          path: localPath,
          name
        });
        
        toast.success("Sound copied to library");
      } catch (e) {
        console.error("Failed to copy sound file", e);
        toast.error("Failed to copy sound to library");
        
        const name = originalName || path.split('\\').pop()?.split('/').pop()?.replace(/\.[^/.]+$/, "") || "Audio";
        this.updateButton(id, { path, name });
      }
    },

    updateButtonByCoords(page: number, x: number, y: number, path: string) {
      const startId = page * this.itemsPerPage;
      const buttonIndex = startId + (y * 4) + x;
      this.updateButtonWithCopy(buttonIndex, path);
    },

    async saveSettings() {
      try {
        const store = await Store.load("store.bin");
        const buttonsToSave = this.buttons.map(b => ({
            id: b.id,
            path: b.path,
            name: b.name,
            volume: b.volume,
            color: b.color
        }));
        await store.set("buttons", buttonsToSave);
        await store.set("masterVolume", this.masterVolume);
        await store.set("currentDevice", this.currentDevice);
        await store.set("totalPages", this.totalPages);
        await store.set("pageNames", this.pageNames);
        await store.set("reducedMotion", this.reducedMotion);
        await store.set("accentColor", this.accentColor);
        await store.set("titlebarStyle", this.titlebarStyle);
        await store.set("controlsSide", this.controlsSide);
        await store.save();
      } catch (e) {
        console.error("Failed to save settings", e);
      }
    },

    async loadSettings() {
      try {
        const store = await Store.load("store.bin");
        const savedButtons = await store.get<any[]>("buttons");
        const savedTotalPages = await store.get<number>("totalPages");
        const savedPageNames = await store.get<string[]>("pageNames");
        const savedReducedMotion = await store.get<boolean>("reducedMotion");
        const savedAccentColor = await store.get<string>("accentColor");
        const savedTitlebarStyle = await store.get<string>("titlebarStyle");
        const savedControlsSide = await store.get<string>("controlsSide");
        
        if (savedTotalPages) {
          this.totalPages = savedTotalPages;
        }

        if (savedPageNames) {
          this.pageNames = savedPageNames;
        } else {
          this.pageNames = Array.from({ length: this.totalPages }, (_, i) => `Page ${i + 1}`);
        }

        if (savedReducedMotion !== null && savedReducedMotion !== undefined) {
          this.reducedMotion = savedReducedMotion;
        }

        if (savedAccentColor) {
          this.accentColor = savedAccentColor;
        }

        if (savedTitlebarStyle === "windows" || savedTitlebarStyle === "mac") {
          this.titlebarStyle = savedTitlebarStyle;
        }

        if (savedControlsSide === "left" || savedControlsSide === "right") {
          this.controlsSide = savedControlsSide;
        }

        const totalButtons = this.itemsPerPage * this.totalPages;
        const buttons = [] as SoundButton[];

        for (let i = 0; i < totalButtons; i++) {
          const saved = savedButtons?.find(sb => sb.id === i);
          buttons.push({
            id: i,
            path: saved?.path || null,
            name: saved?.name || `Button ${i + 1}`,
            volume: saved?.volume ?? 0.5,
            color: saved?.color || "bg-secondary",
            isPaused: false,
            activeInstances: 0
          });
        }
        this.buttons = buttons;
        
        const savedVolume = await store.get<number>("masterVolume");
        if (savedVolume !== null && savedVolume !== undefined) {
          this.masterVolume = savedVolume;
          await invoke("update_master_volume", { volume: savedVolume });
        }
        
        const savedDevice = await store.get<string>("currentDevice");
        if (savedDevice) {
          await this.setOutputDevice(savedDevice);
        }
      } catch (e) {
        console.error("Failed to load settings", e);
      }
    }
  },
});
