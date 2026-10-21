<script setup lang="ts">
import { h } from "vue";
import type { InputProps } from "../data/types";
import { getIconClass } from "../../../../../../hooks/icon.svelte";

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
        <i v-if="icon" :class="getIconClass(icon)" aria-hidden="true" />
        <InputField />
    </div>
</template>
