<script setup lang="ts">
import { ref } from "vue";
import type { CheckboxProps } from "../data/types";
import { StyleComponent } from "../../../../data/style.component";

const { id, name, checked: initialChecked = false, disabled, label, required, onchange } = defineProps<CheckboxProps>();
const checked = ref(initialChecked);

function handleChange(e: Event) {
    checked.value = (e.target as HTMLInputElement).checked;
    onchange?.(checked.value);
}
</script>

<template>
    <label :class="[StyleComponent.checkbox.root, disabled ? StyleComponent.checkbox.disabled : '']" :for="id">
        <input
            type="checkbox"
            :id="id"
            :name="name"
            :checked="checked"
            :disabled="disabled"
            :required="required"
            @change="handleChange"
            :class="StyleComponent.checkbox.input"
        />
        <span :class="StyleComponent.checkbox.box">
            <svg v-if="checked" viewBox="0 0 10 8" fill="none" aria-hidden="true" :class="StyleComponent.checkbox.tick">
                <path d="M1 4l3 3 5-6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
        </span>
        <span v-if="label" :class="StyleComponent.checkbox.label">{{ label }}</span>
    </label>
</template>
