<script setup lang="ts">
import { h } from "vue";
import type { InputProps } from "../data/types";

const { icon, type, required, disabled, placeholder, value } = defineProps<InputProps>();

// snippet: componente funcional interno reutilizável
const InputField = () =>
    h("input", {
        type,
        required,
        disabled,
        placeholder,
        value,
        "data-name": "label",
    });
</script>

<template>
    <div v-if="icon" class="flex items-center">
        <img :src="`/icons/${icon}`" alt="icon" />
        <InputField />
    </div>
    <InputField v-else />
</template>
