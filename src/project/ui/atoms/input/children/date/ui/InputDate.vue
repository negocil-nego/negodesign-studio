<script setup lang="ts">
import { computed, ref, useId } from "vue";
import Icon from "../../../../icon/ui/Icon.vue";
import Popover from "../../../../popover/ui/Popover.vue";
import { StyleComponent } from "../../../../../../data/style.component";
import { useI18n } from "../../../../../../data/i18n/vue";
import {
  computeCalendarMonth,
  formatLongDate,
  stepCalendarMonth,
} from "../../../../../../hooks/date-picker.hook";
import type { InputDateProps } from "../data/types";

const props = defineProps<InputDateProps & { value?: string; onselect?: (date: string) => void }>();
const emit = defineEmits<{ (e: "update:value", v: string): void; (e: "select", v: string): void }>();
const popoverId = useId();
const { locale, t } = useI18n();
const style = StyleComponent.input.date;

const year = ref(2023);
const month = ref(10);
const day = ref(21);
const dateStr = ref(props.value ?? formatLongDate(locale.value, 2023, 10, 21));

const cal = computed(() => computeCalendarMonth(locale.value, year.value, month.value));

function changeMonth(delta: number) {
  const next = stepCalendarMonth(year.value, month.value, delta);
  year.value = next.year;
  month.value = next.month;
}

function handleSelect(selectedDay: number) {
  day.value = selectedDay;
  const formatted = formatLongDate(locale.value, year.value, month.value, selectedDay);
  dateStr.value = formatted;
  emit("update:value", formatted);
  emit("select", formatted);
  props.onselect?.(formatted);
}
</script>

<template>
  <div :class="style.root">
    <Popover :id="popoverId" role="dialog" :ariaLabel="labelProps?.label ?? t('input.date.selectDate')">
      <template #trigger>
        <button type="button" :id="inputProps?.id" :disabled="inputProps?.disabled" :popovertarget="popoverId"
          aria-haspopup="dialog" :class="style.trigger">
          <div :class="style.triggerBody">
            <span v-if="labelProps" :class="style.triggerLabel">{{ labelProps.label }}</span>
            <div :class="style.valueRow">
              <span :class="style.valueIcon"><Icon name="Calendar" /></span>
              <span :class="style.value">{{ dateStr }}</span>
            </div>
          </div>
        </button>
      </template>
      <template #panel>
        <div :class="style.panel">
          <div :class="style.panelHeader">
            <button type="button" @click="changeMonth(-1)" :class="style.navButton"><Icon name="ArrowLeft" /></button>
            <span :class="style.monthTitle">{{ cal.monthName }} {{ year }}</span>
            <button type="button" @click="changeMonth(1)" :class="style.navButton"><Icon name="ArrowRight" /></button>
          </div>
          <div :class="style.weekdays">
            <span v-for="wd in cal.weekdays" :key="wd">{{ wd }}</span>
          </div>
          <div :class="style.daysGrid">
            <span v-for="(pad, idx) in cal.paddingDays" :key="`p-${idx}`" :class="style.paddingDay">{{ pad }}</span>
            <button v-for="d in cal.days" :key="d" type="button" :popovertarget="popoverId" popovertargetaction="hide"
              @click="handleSelect(d)" :class="`${style.day} ${d === day ? style.daySelected : style.dayDefault}`">{{ d }}</button>
          </div>
        </div>
      </template>
    </Popover>
  </div>
</template>
