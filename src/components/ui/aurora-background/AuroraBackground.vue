<script setup lang="ts">
import { cn } from "@inspira-ui/plugins";
import { computed } from "vue";

interface AuroraBackgroundProps {
    radialGradient?: boolean;
    reducedMotion?: boolean;
    accentColor?: string;
    class?: string;
}

const props = withDefaults(defineProps<AuroraBackgroundProps>(), {
    radialGradient: true,
    reducedMotion: false,
    accentColor: "Default",
});

const AURORA_PALETTES: Record<string, string[]> = {
    Default: ["#3b82f6", "#a5b4fc", "#93c5fd", "#ddd6fe", "#60a5fa"],
    Indigo: ["#4f46e5", "#a5b4fc", "#818cf8", "#c7d2fe", "#6366f1"],
    Violet: ["#7c3aed", "#c4b5fd", "#a78bfa", "#ddd6fe", "#8b5cf6"],
    Fuchsia: ["#c026d3", "#f0abfc", "#e879f9", "#f5d0fe", "#d946ef"],
    Rose: ["#e11d48", "#fda4af", "#fb7185", "#fecdd3", "#f43f5e"],
    Crimson: ["#dc2626", "#fca5a5", "#f87171", "#fecaca", "#ef4444"],
    Orange: ["#ea580c", "#fdba74", "#fb923c", "#fed7aa", "#f97316"],
    Amber: ["#d97706", "#fcd34d", "#fbbf24", "#fde68a", "#f59e0b"],
    Lime: ["#65a30d", "#bef264", "#a3e635", "#d9f99d", "#84cc16"],
    Emerald: ["#059669", "#6ee7b7", "#34d399", "#a7f3d0", "#10b981"],
    Cyan: ["#0891b2", "#67e8f9", "#22d3ee", "#a5f3fc", "#06b6d4"],
    Sky: ["#0284c7", "#7dd3fc", "#38bdf8", "#bae6fd", "#0ea5e9"],
};

const auroraColors = computed(() => {
    return AURORA_PALETTES[props.accentColor] || AURORA_PALETTES["Default"]!;
});

const styles = computed(() => {
    const [c1, c2, c3, c4, c5] = auroraColors.value;
    return {
        "--aurora": `repeating-linear-gradient(100deg,${c1}_10%,${c2}_15%,${c3}_20%,${c4}_25%,${c5}_30%)`,
        "--dark-gradient":
            "repeating-linear-gradient(100deg,#000_0%,#000_7%,transparent_10%,transparent_12%,#000_16%)",
        "--white-gradient":
            "repeating-linear-gradient(100deg,#fff_0%,#fff_7%,transparent_10%,transparent_12%,#fff_16%)",

        "--aurora-c1": c1,
        "--aurora-c2": c2,
        "--aurora-c3": c3,
        "--aurora-c4": c4,
        "--aurora-c5": c5,
        "--black": "#000",
        "--white": "#fff",
        "--transparent": "transparent",
        "--animate-aurora": props.reducedMotion
            ? "none"
            : "aurora 60s linear infinite",
    };
});
</script>

<template>
    <main>
        <div
            v-bind="props"
            :class="
                cn(
                    `transition-bg relative flex h-screen flex-col items-center justify-center bg-zinc-50 text-slate-950 dark:bg-zinc-900`,
                    props.class,
                )
            "
        >
            <div :style="styles" class="absolute inset-0 overflow-hidden">
                <div
                    :class="
                        cn(
                            `pointer-events-none absolute -inset-2.5 [background-image:var(--white-gradient),var(--aurora)] bg-size-[300%,200%] bg-position-[50%_50%,50%_50%] opacity-50 blur-[10px] invert filter will-change-transform [--aurora:repeating-linear-gradient(100deg,var(--aurora-c1)_10%,var(--aurora-c2)_15%,var(--aurora-c3)_20%,var(--aurora-c4)_25%,var(--aurora-c5)_30%)] [--dark-gradient:repeating-linear-gradient(100deg,var(--black)_0%,var(--black)_7%,var(--transparent)_10%,var(--transparent)_12%,var(--black)_16%)] [--white-gradient:repeating-linear-gradient(100deg,var(--white)_0%,var(--white)_7%,var(--transparent)_10%,var(--transparent)_12%,var(--white)_16%)] after:absolute after:inset-0 after:[background-image:var(--white-gradient),var(--aurora)] after:bg-size-[200%,100%] after:bg-fixed after:mix-blend-difference after:content-[''] dark:[background-image:var(--dark-gradient),var(--aurora)] after:dark:[background-image:var(--dark-gradient),var(--aurora)]`,
                            !props.reducedMotion && 'after:animate-aurora',
                            props.radialGradient &&
                                `mask-[radial-gradient(ellipse_at_100%_0%,black_10%,var(--transparent)_70%)]`,
                        )
                    "
                />
                <div
                    class="absolute top-0 left-0 h-screen w-screen dark:bg-black/60 bg-black/0 blur-sm"
                ></div>
            </div>
            <slot />
        </div>
    </main>
</template>
