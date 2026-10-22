<script setup lang="ts">
import { ref } from "vue";
import type { RadioProps } from "../data/types";

const { name, options, value: initialValue = "", disabled, onchange } = defineProps<RadioProps>();
const selected = ref(initialValue);

function handleChange(optionValue: string) {
    selected.value = optionValue;
    onchange?.(optionValue);
}
</script>

<template>
    <fieldset class="radio-group" :disabled="disabled">
        <label
            v-for="option in options"
            :key="option.value"
            :class="['radio-item', { 'radio-disabled': option.disabled || disabled }]"
        >
            <input
                type="radio"
                :name="name"
                :value="option.value"
                :checked="selected === option.value"
                :disabled="option.disabled || disabled"
                @change="handleChange(option.value)"
                class="radio-input"
            />
            <span class="radio-dot" />
            <span class="radio-label">{{ option.label }}</span>
        </label>
    </fieldset>
</template>
