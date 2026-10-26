<script setup lang="ts">
import { ref } from "vue";
import type { ToggleProps } from "../data/types";
import { StyleComponent } from "../../../../data/style.component";

const { id, name, checked: initialChecked = false, disabled, label, onchange } = defineProps<ToggleProps>();
const checked = ref(initialChecked);

function handleChange() {
    checked.value = !checked.value;
    onchange?.(checked.value);
}
</script>

<template>
    <label :class="StyleComponent.toggle.root" :for="id">
        <button
            type="button"
            role="switch"
            :id="id"
            :name="name"
            :aria-checked="checked"
            :disabled="disabled"
            @click="handleChange"
            :class="[StyleComponent.toggle.track, checked ? StyleComponent.toggle.on : '']"
        >
            <span :class="StyleComponent.toggle.thumb" />
        </button>
        <span v-if="label" :class="StyleComponent.toggle.label">{{ label }}</span>
    </label>
</template>
