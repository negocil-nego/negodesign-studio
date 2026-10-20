<script setup lang="ts">
import type { InputProps } from "../data/types";

const { type, required, disabled, placeholder, value } = defineProps<InputProps>();
</script>

<template>
    <input :type="type" :required="required" :disabled="disabled" :placeholder="placeholder" :value="value"
        data-name="label" />
</template>
