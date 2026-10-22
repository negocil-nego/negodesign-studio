<script setup lang="ts">
import { ref, computed, useId } from "vue";
import Icon from "../../../../icon/ui/Icon.vue";
import type { InputDateProps } from "../data/types";

const props = withDefaults(defineProps<InputDateProps & { value?: string }>(), {
  value: "November 21, 2023"
});
const emit = defineEmits<{ select: [date: string] }>();

const popoverId = useId();
const currentYear = ref(2023);
const currentMonthIndex = ref(10);
const selectedDay = ref(21);
const dateStr = ref(props.inputProps?.value ?? props.value);

const monthNames = [
  "January", "February", "March", "April", "May", "June",
  "July", "August", "September", "October", "November", "December"
];

const currentMonthName = computed(() => monthNames[currentMonthIndex.value]);
const daysInMonth = computed(() => new Date(currentYear.value, currentMonthIndex.value + 1, 0).getDate());
const firstDayOfWeek = computed(() => (new Date(currentYear.value, currentMonthIndex.value, 1).getDay() + 6) % 7);
const days = computed(() => Array.from({ length: daysInMonth.value }, (_, i) => i + 1));
const prevMonthDays = computed(() => new Date(currentYear.value, currentMonthIndex.value, 0).getDate());
const paddingDaysBefore = computed(() => Array.from({ length: firstDayOfWeek.value }, (_, i) => prevMonthDays.value - firstDayOfWeek.value + i + 1));

function nextMonth() {
  if (currentMonthIndex.value === 11) {
    currentMonthIndex.value = 0;
    currentYear.value += 1;
  } else {
    currentMonthIndex.value += 1;
  }
}

function prevMonth() {
  if (currentMonthIndex.value === 0) {
    currentMonthIndex.value = 11;
    currentYear.value -= 1;
  } else {
    currentMonthIndex.value -= 1;
  }
}

function selectDate(day: number) {
  selectedDay.value = day;
  dateStr.value = `${currentMonthName.value} ${day}, ${currentYear.value}`;
  emit("select", dateStr.value);
}
</script>

<template>
  <div class="relative w-full">
    <div class="popover w-full">
      <button type="button" :id="props.inputProps?.id" :disabled="props.inputProps?.disabled" :popovertarget="popoverId"
        aria-haspopup="dialog" class="input-form w-full cursor-pointer flex items-center justify-between text-left">
        <div class="flex flex-col w-full min-w-0">
          <span v-if="props.labelProps" class="text-[10px] uppercase font-semibold text-zinc-500 select-none">
            {{ props.labelProps.label }}
          </span>
          <div class="flex items-center gap-2 mt-0.5">
            <span class="text-sm">
              <Icon name="Label" />
            </span>
            <span class="text-sm font-medium truncate">
              {{ dateStr }}
            </span>
          </div>
        </div>
        <div class="flex flex-col text-zinc-400 text-xs ml-2 select-none shrink-0" aria-hidden="true">
          <span><Icon name="HelpSquare" /></span>
          <span><Icon name="HelpSquare" /></span>
        </div>
      </button>

      <div :id="popoverId" popover="auto" role="dialog" :aria-label="props.labelProps?.label ?? 'Pick a date'"
        class="mt-1.5 text-zinc-200 z-50 outline-none">
        <div class="flex items-center justify-between mb-4">
          <button type="button" @click="prevMonth" class="p-1.5 rounded hover:bg-blender-hover cursor-pointer">
            <Icon name="HelpSquare" />
          </button>
          <span class="font-bold text-sm text-zinc-200">{{ currentMonthName }} {{ currentYear }}</span>
          <button type="button" @click="nextMonth" class="p-1.5 rounded hover:bg-blender-hover cursor-pointer">
            <Icon name="HelpSquare" />
          </button>
        </div>

        <div class="grid grid-cols-7 text-center text-xs font-medium text-zinc-500 mb-3">
          <span>Mon</span><span>Tue</span><span>Wed</span><span>Thu</span><span>Fri</span><span>Sat</span><span>Sun</span>
        </div>

        <div class="grid grid-cols-7 gap-y-1.5 text-center text-sm font-medium">
          <span v-for="pad in paddingDaysBefore" :key="'pad-' + pad" class="text-zinc-600 py-1 opacity-40">{{ pad
          }}</span>
          <button v-for="day in days" :key="day" type="button" :popovertarget="popoverId" popovertargetaction="hide"
            @click="selectDate(day)"
            class="w-8 h-8 mx-auto flex items-center justify-center rounded-full transition-all cursor-pointer"
            :class="day === selectedDay ? 'bg-orange-500 text-white font-bold shadow-md' : 'text-zinc-300 hover:bg-blender-hover'">
            {{ day }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>