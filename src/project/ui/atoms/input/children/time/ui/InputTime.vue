<script setup lang="ts">
import { computed, ref, useId, watch } from "vue";
import Icon from "../../../../icon/ui/Icon.vue";
import Popover from "../../../../popover/ui/Popover.vue";
import { StyleComponent } from "../../../../../../data/style.component";
import { useI18n } from "../../../../../../data/i18n/vue";
import { formatTimeDisplay, parseTimeValue, TIME_HOURS, TIME_MINUTES, TIME_PERIODS } from "../../../../../../hooks/time-picker.hook";
import type { InputTimeProps } from "../data/types";

const props = defineProps<InputTimeProps & { value?: string; onselect?: (time: string) => void }>();
const emit = defineEmits<{ (e: "update:value", v: string): void; (e: "select", v: string): void }>();
const popoverId = useId();
const { t } = useI18n();
const style = StyleComponent.input.time;

const selectedHour = ref("02");
const selectedMinute = ref("30");
const selectedPeriod = ref("AM");

watch(() => props.value, (val) => {
  if (val) {
    const parsed = parseTimeValue(val);
    selectedHour.value = parsed.hour;
    selectedMinute.value = parsed.minute;
    selectedPeriod.value = parsed.period;
  }
}, { immediate: true });

const displayTime = computed(() => formatTimeDisplay(selectedHour.value, selectedMinute.value, selectedPeriod.value));

function apply(h = selectedHour.value, m = selectedMinute.value, p = selectedPeriod.value) {
  selectedHour.value = h;
  selectedMinute.value = m;
  selectedPeriod.value = p;
  const formatted = formatTimeDisplay(h, m, p);
  emit("update:value", formatted);
  emit("select", formatted);
  props.onselect?.(formatted);
}

function clearTime(e: MouseEvent) {
  e.stopPropagation();
  selectedHour.value = "";
  selectedMinute.value = "";
  selectedPeriod.value = "AM";
  emit("update:value", "");
  emit("select", "");
  props.onselect?.("");
}
</script>

<template>
  <div :class="style.root">
    <Popover :id="popoverId" role="dialog" :ariaLabel="labelProps?.label ?? t('input.time.selectTime')">
      <template #trigger>
        <button type="button" :id="inputProps?.id" :disabled="inputProps?.disabled" :popovertarget="popoverId"
          aria-haspopup="dialog" :class="style.trigger">
          <div :class="style.triggerBody">
            <span v-if="labelProps" :class="style.triggerLabel">{{ labelProps.label }}</span>
            <div :class="style.valueRow">
              <span :class="style.valueIcon"><Icon name="Time" /></span>
              <span :class="style.value">{{ displayTime }}</span>
            </div>
          </div>
          <button v-if="selectedHour" type="button" :class="style.clearButton" @click="clearTime" :aria-label="t('input.time.clear')">
            <Icon name="Cancel" />
          </button>
        </button>
      </template>
      <template #panel>
        <div :class="style.panel">
          <div :class="style.pickerGrid">
            <div :class="style.pickerColumn">
              <button v-for="h in TIME_HOURS" :key="h" type="button" :class="`${style.pickerItem} ${selectedHour === h ? style.pickerItemSelected : ''}`" @click="apply(h, selectedMinute, selectedPeriod)">{{ h }}</button>
            </div>
            <div :class="style.columnDivider"></div>
            <div :class="style.pickerColumn">
              <button v-for="m in TIME_MINUTES" :key="m" type="button" :class="`${style.pickerItem} ${selectedMinute === m ? style.pickerItemSelected : ''}`" @click="apply(selectedHour, m, selectedPeriod)">{{ m }}</button>
            </div>
            <div :class="style.columnDivider"></div>
            <div :class="style.pickerColumn">
              <button v-for="p in TIME_PERIODS" :key="p" type="button" :class="`${style.pickerItem} ${selectedPeriod === p ? style.pickerItemSelected : ''}`" @click="apply(selectedHour, selectedMinute, p)">{{ p }}</button>
            </div>
          </div>
        </div>
      </template>
    </Popover>
  </div>
</template>
