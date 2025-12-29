<script setup lang="ts">
import { useAudioStore } from "@/stores/audio";
import { storeToRefs } from "pinia";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { onMounted } from "vue";

const audioStore = useAudioStore();
const { devices, currentDevice } = storeToRefs(audioStore);

onMounted(() => {
  audioStore.loadDevices();
});

const onValueChange = (val: string) => {
  audioStore.setOutputDevice(val);
};
</script>

<template>
  <div class="flex items-center space-x-2">
    <span class="text-sm font-medium">Output:</span>
    <Select :model-value="currentDevice" @update:model-value="onValueChange">
      <SelectTrigger class="w-[280px]">
        <SelectValue placeholder="Select audio output" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem v-for="device in devices" :key="device" :value="device">
          {{ device }}
        </SelectItem>
      </SelectContent>
    </Select>
  </div>
</template>
