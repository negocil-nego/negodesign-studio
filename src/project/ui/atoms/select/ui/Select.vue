<script setup lang="ts">
import { ref, watch } from "vue";
import type { SelectProps } from "../data/types";

const { id, name, options, value, placeholder, disabled, required } = defineProps<SelectProps>();
const selected = ref(value ?? "");
watch(() => value, (v) => { selected.value = v ?? ""; });
</script>

<template>
    <div class="input-form">
        <select :id="id" :name="name" v-model="selected" :disabled="disabled" :required="required" class="input-select">
            <option v-if="placeholder" value="" disabled hidden>{{ placeholder }}</option>
            <option v-for="option in options" :key="option.value" :value="option.value" :disabled="option.disabled">
                {{ option.label }}
            </option>
        </select>
    </div>
</template>
