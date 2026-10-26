<script setup lang="ts">
import { ref, computed, useId, watch } from "vue";
import Icon from "../../../../icon/ui/Icon.vue";
import Popover from "../../../../popover/ui/Popover.vue";
import { StyleComponent } from "../../../../../../data/style.component";
import type { InputTimeProps } from "../data/types";

const props = withDefaults(
    defineProps<InputTimeProps & { value?: string; onselect?: (time: string) => void }>(),
    { value: "" },
);

const popoverId = useId();
const style = StyleComponent.input.time;
const selectedHour = ref("02");
const selectedMinute = ref("30");
const selectedPeriod = ref("AM");
const currentTime = ref(props.value);

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
    props.onselect?.(nextVal);
}

function clearTime(e: MouseEvent) {
    e.stopPropagation();
    currentTime.value = "";
    selectedHour.value = "";
    selectedMinute.value = "";
    selectedPeriod.value = "AM";
    props.onselect?.("");
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
    <div :class="style.root">
        <Popover
            :id="popoverId"
            role="dialog"
            :aria-label="props.labelProps?.label ?? 'Pick a time'"
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
                                <Icon name="Time" />
                            </span>
                            <span :class="style.value">{{ displayTime }}</span>
                        </div>
                    </div>

                    <div class="flex items-center gap-1">
                        <button
                            v-if="currentTime"
                            type="button"
                            :class="style.clearButton"
                            @click="clearTime"
                            aria-label="Clear time"
                        >
                            <Icon name="Cancel" />
                        </button>
                    </div>
                </button>
            </template>

            <template #panel>
                <div :class="style.panel">
                    <div :class="style.pickerGrid">
                        <!-- Hours -->
                        <div :class="style.pickerColumn">
                            <button
                                v-for="h in hours"
                                :key="h"
                                type="button"
                                :class="[style.pickerItem, selectedHour === h ? style.pickerItemSelected : '']"
                                @click="() => { selectedHour = h; handleSelect(h, selectedMinute, selectedPeriod); }"
                            >
                                {{ h }}
                            </button>
                        </div>

                        <div :class="style.columnDivider"></div>

                        <!-- Minutes -->
                        <div :class="style.pickerColumn">
                            <button
                                v-for="m in minutes"
                                :key="m"
                                type="button"
                                :class="[style.pickerItem, selectedMinute === m ? style.pickerItemSelected : '']"
                                @click="() => { selectedMinute = m; handleSelect(selectedHour, m, selectedPeriod); }"
                            >
                                {{ m }}
                            </button>
                        </div>

                        <div :class="style.columnDivider"></div>

                        <!-- Period -->
                        <div :class="style.pickerColumn">
                            <button
                                v-for="p in periods"
                                :key="p"
                                type="button"
                                :class="[style.pickerItem, selectedPeriod === p ? style.pickerItemSelected : '']"
                                @click="() => { selectedPeriod = p; handleSelect(selectedHour, selectedMinute, p); }"
                            >
                                {{ p }}
                            </button>
                        </div>
                    </div>
                </div>
            </template>
        </Popover>
    </div>
</template>
