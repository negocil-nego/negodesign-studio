<script setup lang="ts">
import { ref } from "vue";
import type { ToggleProps } from "../data/types";

const { id, name, checked: initialChecked = false, disabled, label, onchange } = defineProps<ToggleProps>();
const checked = ref(initialChecked);

function handleChange() {
    checked.value = !checked.value;
    onchange?.(checked.value);
}
</script>

<template>
    <label class="toggle-root" :for="id">
        <button
            type="button"
            role="switch"
            :id="id"
            :name="name"
            :aria-checked="checked"
            :disabled="disabled"
            @click="handleChange"
            :class="['toggle-track', { 'toggle-on': checked }]"
        >
            <span class="toggle-thumb" />
        </button>
        <span v-if="label" class="toggle-label">{{ label }}</span>
    </label>
</template>
