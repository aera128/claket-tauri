<script setup lang="ts">
import { cn } from "@/lib/utils";
import { useAudioStore } from "@/stores/audio";
import { computed } from "vue";

const store = useAudioStore();

interface AuroraBackgroundProps {
  className?: string;
  showRadialGradient?: boolean;
  mode?: 'original' | 'optimized';
}

const props = withDefaults(defineProps<AuroraBackgroundProps>(), {
  showRadialGradient: true,
  mode: 'optimized',
});

const styles = computed(() => {
  return {
    "--aurora":
      "repeating-linear-gradient(100deg,#3b82f6_10%,#a5b4fc_15%,#93c5fd_20%,#ddd6fe_25%,#60a5fa_30%)",
    "--dark-gradient":
      "repeating-linear-gradient(100deg,#000_0%,#000_7%,transparent_10%,transparent_12%,#000_16%)",
    "--white-gradient":
      "repeating-linear-gradient(100deg,#fff_0%,#fff_7%,transparent_10%,transparent_12%,#fff_16%)",

    "--blue-300": "#93c5fd",
    "--blue-400": "#60a5fa",
    "--blue-500": "#3b82f6",
    "--indigo-300": "#a5b4fc",
    "--violet-200": "#ddd6fe",
    "--black": "#000",
    "--white": "#fff",
    "--transparent": "transparent",
    "--animate-aurora": "aurora 60s linear infinite",
  };
});
</script>

<template>
  <main>
    <div
      :class="cn(
        'transition-bg relative flex min-h-screen flex-col bg-zinc-50 text-slate-950 dark:bg-zinc-950 dark:text-white transition-colors duration-500',
        props.className,
      )"
    >
      <!-- ORIGINAL MODE (blur 10px - High GPU) -->
      <div v-if="!store.reducedMotion && mode === 'original'" class="fixed inset-0 overflow-hidden pointer-events-none z-0">
        <div
          :style="styles"
          class="absolute inset-0 overflow-hidden"
        >
          <div
            :class="
              cn(
                `after:animate-aurora pointer-events-none absolute -inset-2.5 [background-image:var(--white-gradient),var(--aurora)] bg-size-[300%,200%] bg-position-[50%_50%,50%_50%] opacity-50 blur-[10px] invert filter will-change-transform [--aurora:repeating-linear-gradient(100deg,var(--blue-500)_10%,var(--indigo-300)_15%,var(--blue-300)_20%,var(--violet-200)_25%,var(--blue-400)_30%)] [--dark-gradient:repeating-linear-gradient(100deg,var(--black)_0%,var(--black)_7%,var(--transparent)_10%,var(--transparent)_12%,var(--black)_16%)] [--white-gradient:repeating-linear-gradient(100deg,var(--white)_0%,var(--white)_7%,var(--transparent)_10%,var(--transparent)_12%,var(--white)_16%)] after:absolute after:inset-0 after:[background-image:var(--white-gradient),var(--aurora)] after:bg-size-[200%,100%] after:bg-fixed after:mix-blend-difference after:content-[''] dark:[background-image:var(--dark-gradient),var(--aurora)] dark:invert-0 after:dark:[background-image:var(--dark-gradient),var(--aurora)]`,
                props.showRadialGradient &&
                  `mask-[radial-gradient(ellipse_at_100%_0%,black_10%,var(--transparent)_70%)]`,
              )
            "
          />
        </div>
      </div>

      <!-- OPTIMIZED MODE (blur 4px - Same effect, ~60% less GPU) -->
      <div v-if="!store.reducedMotion && mode === 'optimized'" class="fixed inset-0 overflow-hidden pointer-events-none z-0">
        <div
          :style="styles"
          class="absolute inset-0 overflow-hidden"
        >
          <div
            :class="
              cn(
                `after:animate-aurora pointer-events-none absolute -inset-2.5 [background-image:var(--white-gradient),var(--aurora)] bg-size-[300%,200%] bg-position-[50%_50%,50%_50%] opacity-50 blur-[4px] invert filter will-change-transform [--aurora:repeating-linear-gradient(100deg,var(--blue-500)_10%,var(--indigo-300)_15%,var(--blue-300)_20%,var(--violet-200)_25%,var(--blue-400)_30%)] [--dark-gradient:repeating-linear-gradient(100deg,var(--black)_0%,var(--black)_7%,var(--transparent)_10%,var(--transparent)_12%,var(--black)_16%)] [--white-gradient:repeating-linear-gradient(100deg,var(--white)_0%,var(--white)_7%,var(--transparent)_10%,var(--transparent)_12%,var(--white)_16%)] after:absolute after:inset-0 after:[background-image:var(--white-gradient),var(--aurora)] after:bg-size-[200%,100%] after:bg-fixed after:mix-blend-difference after:content-[''] dark:[background-image:var(--dark-gradient),var(--aurora)] dark:invert-0 after:dark:[background-image:var(--dark-gradient),var(--aurora)]`,
                props.showRadialGradient &&
                  `mask-[radial_gradient(ellipse_at_100%_0%,black_10%,var(--transparent)_70%)]`,
              )
            "
          />
        </div>
      </div>

      <div class="relative z-10">
        <slot />
      </div>
    </div>
  </main>
</template>

<style>
@keyframes aurora {
  from { background-position: 50% 50%, 50% 50%; }
  to { background-position: 350% 50%, 350% 50%; }
}

.animate-aurora {
  animation: aurora 60s linear infinite;
}
</style>
