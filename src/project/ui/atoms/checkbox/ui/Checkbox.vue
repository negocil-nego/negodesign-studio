<script setup lang="ts">
import { ref } from "vue";
import type { CheckboxProps } from "../data/types";

const { id, name, checked: initialChecked = false, disabled, label, required, onchange } = defineProps<CheckboxProps>();
const checked = ref(initialChecked);

function handleChange(e: Event) {
    checked.value = (e.target as HTMLInputElement).checked;
    onchange?.(checked.value);
}
</script>

<template>
    <label :class="['checkbox-root', { 'checkbox-disabled': disabled }]" :for="id">
        <input
            type="checkbox"
            :id="id"
            :name="name"
            :checked="checked"
            :disabled="disabled"
            :required="required"
            @change="handleChange"
            class="checkbox-input"
        />
        <span class="checkbox-box">
            <svg v-if="checked" viewBox="0 0 10 8" fill="none" aria-hidden="true" class="checkbox-tick">
                <path d="M1 4l3 3 5-6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
        </span>
        <span v-if="label" class="checkbox-label">{{ label }}</span>
    </label>
</template>
