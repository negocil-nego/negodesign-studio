<script setup lang="ts">
import { ref, computed, useId } from "vue";
import Icon from "../../../../icon/ui/Icon.vue";
import Popover from "../../../../popover/ui/Popover.vue";
import { StyleComponent } from "../../../../../../data/style.component";
import type { InputDateProps } from "../data/types";

const props = withDefaults(
    defineProps<InputDateProps & { value?: string; onselect?: (date: string) => void }>(),
    { value: "November 21, 2023" },
);

const popoverId = useId();
const style = StyleComponent.input.date;
const year = ref(2023);
const month = ref(10); // 10 = November
const day = ref(21);
const dateStr = ref(props.value);

const months = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

const monthName = computed(() => months[month.value]);
const daysInMonth = computed(() => new Date(year.value, month.value + 1, 0).getDate());
const startDay = computed(() => (new Date(year.value, month.value, 1).getDay() + 6) % 7);
const days = computed(() => Array.from({ length: daysInMonth.value }, (_, i) => i + 1));
const prevDaysCount = computed(() => new Date(year.value, month.value, 0).getDate());
const paddingDays = computed(() =>
    Array.from({ length: startDay.value }, (_, i) => prevDaysCount.value - startDay.value + i + 1),
);

function changeMonth(delta: number) {
    const next = month.value + delta;
    if (next > 11) {
        month.value = 0;
        year.value++;
    } else if (next < 0) {
        month.value = 11;
        year.value--;
    } else {
        month.value = next;
    }
}

function handleSelect(selectedDay: number) {
    day.value = selectedDay;
    dateStr.value = `${monthName.value} ${selectedDay}, ${year.value}`;
    props.onselect?.(dateStr.value);
}
</script>

<template>
    <div :class="style.root">
        <Popover
            :id="popoverId"
            role="dialog"
            :aria-label="props.labelProps?.label ?? 'Pick a date'"
        >
            <template #trigger>
                <button
                    type="button"
                    :id="props.inputProps?.id"
                    :disabled="props.inputProps?.disabled"
                    :popovertarget="popoverId"
                    aria-haspopup="dialog"
                    :class="style.trigger"
                >
                    <div :class="style.triggerBody">
                        <span v-if="props.labelProps" :class="style.triggerLabel">
                            {{ props.labelProps.label }}
                        </span>
                        <div :class="style.valueRow">
                            <span :class="style.valueIcon">
                                <Icon name="Calendar" />
                            </span>
                            <span :class="style.value">{{ dateStr }}</span>
                        </div>
                    </div>
                </button>
            </template>

            <template #panel>
                <div :class="style.panel">
                    <div :class="style.panelHeader">
                        <button type="button" @click="changeMonth(-1)" :class="style.navButton">
                            <Icon name="ArrowLeft" />
                        </button>
                        <span :class="style.monthTitle">{{ monthName }} {{ year }}</span>
                        <button type="button" @click="changeMonth(1)" :class="style.navButton">
                            <Icon name="ArrowRight" />
                        </button>
                    </div>

                    <div :class="style.weekdays">
                        <span>Mon</span><span>Tue</span><span>Wed</span><span>Thu</span><span>Fri</span><span>Sat</span><span>Sun</span>
                    </div>

                    <div :class="style.daysGrid">
                        <span v-for="(pad, idx) in paddingDays" :key="`pad-${idx}`" :class="style.paddingDay">{{ pad }}</span>
                        <button
                            v-for="d in days"
                            :key="d"
                            type="button"
                            :popovertarget="popoverId"
                            popovertargetaction="hide"
                            @click="handleSelect(d)"
                            :class="[style.day, d === day ? style.daySelected : style.dayDefault]"
                        >
                            {{ d }}
                        </button>
                    </div>
                </div>
            </template>
        </Popover>
    </div>
</template>
