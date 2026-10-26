<script setup lang="ts">
import { ref } from "vue";
import type { RadioProps } from "../data/types";
import { StyleComponent } from "../../../../data/style.component";

const { name, options, value: initialValue = "", disabled, onchange } = defineProps<RadioProps>();
const selected = ref(initialValue);

function handleChange(optionValue: string) {
    selected.value = optionValue;
    onchange?.(optionValue);
}
</script>

<template>
    <fieldset :class="StyleComponent.radio.group" :disabled="disabled">
        <label
            v-for="option in options"
            :key="option.value"
            :class="[StyleComponent.radio.item, (option.disabled || disabled) ? StyleComponent.radio.disabled : '']"
        >
            <input
                type="radio"
                :name="name"
                :value="option.value"
                :checked="selected === option.value"
                :disabled="option.disabled || disabled"
                @change="handleChange(option.value)"
                :class="StyleComponent.radio.input"
            />
            <span :class="StyleComponent.radio.dot" />
            <span :class="StyleComponent.radio.label">{{ option.label }}</span>
        </label>
    </fieldset>
</template>
