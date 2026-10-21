<script setup lang="ts">
import { h } from "vue";
import type { InputProps } from "../data/types";

const { icon, type, required, disabled, placeholder, value } = defineProps<InputProps>();

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
    <div class="input-form">
        <img v-if="icon" :src="`/icons/${icon}`" alt="icon" />
        <InputField />
    </div>
</template>
