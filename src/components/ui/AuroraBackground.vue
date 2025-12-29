<script setup lang="ts">
import { cn } from "@/lib/utils";
import { useAudioStore } from "@/stores/audio";

const store = useAudioStore();

interface AuroraBackgroundProps {
  className?: string;
  showRadialGradient?: boolean;
}

withDefaults(defineProps<AuroraBackgroundProps>(), {
  showRadialGradient: true,
});
</script>

<template>
  <main>
    <div
      :class="cn(
        'transition-bg relative flex min-h-screen flex-col bg-zinc-50 text-slate-950 dark:bg-zinc-950 dark:text-white transition-colors duration-500',
        $props.className,
      )"
    >
      <div class="fixed inset-0 overflow-hidden pointer-events-none z-0">
        <div
          :class="cn(
            `
            pointer-events-none absolute -inset-[10px] opacity-30 blur-[10px] invert filter will-change-transform dark:invert-0 dark:opacity-20
            after:absolute after:inset-0 after:mix-blend-difference after:content-['']
            [background-image:var(--white-gradient),var(--aurora)] [background-size:300%,_200%] [background-position:50%_50%,50%_50%]
            after:[background-image:var(--white-gradient),var(--aurora)] after:[background-size:200%,_100%] after:[background-attachment:fixed]
            dark:[background-image:var(--dark-gradient),var(--aurora)] after:dark:[background-image:var(--dark-gradient),var(--aurora)]
            ${!store.reducedMotion ? 'after:animate-aurora' : ''}
            `,
            showRadialGradient && `[mask-image:radial-gradient(ellipse_at_100%_0%,black_10%,transparent_70%)]`,
          )"
          style="
            --aurora: repeating-linear-gradient(100deg, #3b82f6 10%, #a5b4fc 15%, #93c5fd 20%, #ddd6fe 25%, #60a5fa 30%);
            --dark-gradient: repeating-linear-gradient(100deg, #000 0%, #000 7%, transparent 10%, transparent 12%, #000 16%);
            --white-gradient: repeating-linear-gradient(100deg, #fff 0%, #fff 7%, transparent 10%, transparent 12%, #fff 16%);
            --blue-300: #93c5fd;
            --blue-400: #60a5fa;
            --blue-500: #3b82f6;
            --indigo-300: #a5b4fc;
            --violet-200: #ddd6fe;
            --black: #000;
            --white: #fff;
            --transparent: transparent;
          "
        ></div>
      </div>
      <div class="relative z-10">
        <slot />
      </div>
    </div>
  </main>
</template>

<style>
@keyframes aurora-kf {
  from {
    background-position: 50% 50%, 50% 50%;
  }
  to {
    background-position: 350% 50%, 350% 50%;
  }
}

.aurora-animate-direct {
  animation: aurora-kf 60s linear infinite;
}

.aurora-animate-direct::after {
  animation: aurora-kf 60s linear infinite;
}
</style>
