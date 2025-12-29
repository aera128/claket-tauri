<script lang="ts" setup>
import { Toaster as Sonner } from "vue-sonner"
import { useDark } from "@vueuse/core"

const isDark = useDark()
</script>

<template>
  <Sonner
    class="toaster group"
    position="top-right"
    :theme="isDark ? 'dark' : 'light'"
    :rich-colors="true"
    :close-button="true"
    :toast-options="{
      classes: {
        toast: 'group-[.toaster]:bg-background group-[.toaster]:text-foreground group-[.toaster]:border-border group-[.toaster]:shadow-lg',
        description: 'group-[.toast]:text-muted-foreground',
        actionButton: 'group-[.toast]:bg-primary group-[.toast]:text-primary-foreground',
        cancelButton: 'group-[.toast]:bg-muted group-[.toast]:text-muted-foreground',
      },
    }"
  />
</template>

