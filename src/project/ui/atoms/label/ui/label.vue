<script setup lang="ts">
import type { LabelProps } from "../data/types";

const { label, for: htmlFor } = defineProps<LabelProps>();
</script>

<template>
    <label :for="htmlFor" data-name="label">{{ label }}</label>
</template>