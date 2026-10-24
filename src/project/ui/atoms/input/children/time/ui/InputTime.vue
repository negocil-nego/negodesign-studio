<script setup lang="ts">
import { ref, computed, useId, watch } from "vue";
import Icon from "../../../../icon/ui/Icon.vue";
import type { InputTimeProps } from "../data/types";

const props = withDefaults(
  defineProps<InputTimeProps & { value?: string }>(),
  {
    value: "",
  },
);
const emit = defineEmits<{ select: [time: string] }>();

const popoverId = useId();
const selectedHour = ref("02");
const selectedMinute = ref("30");
const selectedPeriod = ref("AM");
const currentTime = ref(props.inputProps?.value ?? props.value);

watch(
  () => props.value,
  (newVal) => {
    if (newVal) {
      const match = newVal.match(/(\d{2}):(\d{2})\s?(AM|PM)/i);
      if (match) {
        selectedHour.value = match[1];
        selectedMinute.value = match[2];
        selectedPeriod.value = match[3].toUpperCase();
        currentTime.value = newVal;
      }
    }
  },
  { immediate: true },
);

const displayTime = computed(() =>
  selectedHour.value && selectedMinute.value
    ? `${selectedHour.value}:${selectedMinute.value} ${selectedPeriod.value}`
    : "-- : --",
);

function handleSelect(h: string, m: string, p: string) {
  const nextVal = `${h}:${m} ${p}`;
  currentTime.value = nextVal;
  emit("select", nextVal);
}

function clearTime(e: MouseEvent) {
  e.stopPropagation();
  currentTime.value = "";
  selectedHour.value = "";
  selectedMinute.value = "";
  selectedPeriod.value = "AM";
  emit("select", "");
}

const hours = Array.from({ length: 12 }, (_, i) =>
  (i + 1).toString().padStart(2, "0"),
);
const minutes = Array.from({ length: 60 / 5 }, (_, i) =>
  (i * 5).toString().padStart(2, "0"),
);
const periods = ["AM", "PM"];
</script>

<template>
  <div class="relative w-full">
    <div class="popover w-full">
      <button
        type="button"
        :id="props.inputProps?.id"
        :disabled="props.inputProps?.disabled"
        :popovertarget="popoverId"
        aria-haspopup="dialog"
        class="flex w-full min-w-0 items-center justify-between rounded-md border border-blender-border bg-blender-surface px-3 py-2 text-sm text-zinc-200 focus-within:border-blue-500 focus-within:ring-1 focus-within:ring-blue-500 cursor-pointer text-left"
      >
        <div class="flex flex-col w-full min-w-0 items-center">
          <span
            v-if="props.labelProps"
            class="text-[10px] mt-2 uppercase font-semibold text-zinc-500 select-none"
          >
            {{ props.labelProps.label }}
          </span>
          <div class="flex items-center gap-2 mt-0.5">
            <span class="text-sm text-zinc-400">
              <Icon name="Time" />
            </span>
            <span class="text-sm font-medium truncate">
              {{ displayTime }}
            </span>
          </div>
        </div>

        <div class="flex items-center gap-1">
          <button
            v-if="currentTime"
            type="button"
            class="p-1 rounded-full hover:bg-blender-hover text-zinc-500 hover:text-zinc-300 transition-colors"
            @click="clearTime"
            aria-label="Clear time"
          >
            <Icon name="Cancel" />
          </button>
          <div
            class="flex flex-col text-zinc-400 text-[10px] ml-1 select-none shrink-0"
            aria-hidden="true"
          >
            <span>▲</span>
            <span>▼</span>
          </div>
        </div>
      </button>

      <div
        :id="popoverId"
        popover="auto"
        role="dialog"
        :aria-label="props.labelProps?.label ?? 'Pick a time'"
        class="mt-1.5 p-3 rounded-lg border border-blender-border bg-blender-surface shadow-xl z-50 outline-none w-56"
      >
        <div class="flex justify-between gap-2 h-48">
          <!-- Hours -->
          <div
            class="flex-1 flex flex-col overflow-y-auto no-scrollbar snap-y snap-mandatory"
          >
            <button
              v-for="h in hours"
              :key="h"
              type="button"
              class="flex items-center justify-center py-2 text-sm font-medium transition-colors cursor-pointer snap-center shrink-0"
              :class="
                selectedHour === h
                  ? 'bg-blue-600 text-white rounded-md'
                  : 'text-zinc-500 hover:text-zinc-300'
              "
              @click="
                () => {
                  selectedHour = h;
                  handleSelect(h, selectedMinute, selectedPeriod);
                }
              "
            >
              {{ h }}
            </button>
          </div>

          <div class="w-[1px] bg-blender-border/50 self-stretch my-2"></div>

          <!-- Minutes -->
          <div
            class="flex-1 flex flex-col overflow-y-auto no-scrollbar snap-y snap-mandatory"
          >
            <button
              v-for="m in minutes"
              :key="m"
              type="button"
              class="flex items-center justify-center py-2 text-sm font-medium transition-colors cursor-pointer snap-center shrink-0"
              :class="
                selectedMinute === m
                  ? 'bg-blue-600 text-white rounded-md'
                  : 'text-zinc-500 hover:text-zinc-300'
              "
              @click="
                () => {
                  selectedMinute = m;
                  handleSelect(selectedHour, m, selectedPeriod);
                }
              "
            >
              {{ m }}
            </button>
          </div>

          <div class="w-[1px] bg-blender-border/50 self-stretch my-2"></div>

          <!-- Period -->
          <div
            class="flex-1 flex flex-col overflow-y-auto no-scrollbar snap-y snap-mandatory"
          >
            <button
              v-for="p in periods"
              :key="p"
              type="button"
              class="flex items-center justify-center py-2 text-sm font-medium transition-colors cursor-pointer snap-center shrink-0"
              :class="
                selectedPeriod === p
                  ? 'bg-blue-600 text-white rounded-md'
                  : 'text-zinc-500 hover:text-zinc-300'
              "
              @click="
                () => {
                  selectedPeriod = p;
                  handleSelect(selectedHour, selectedMinute, p);
                }
              "
            >
              {{ p }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
