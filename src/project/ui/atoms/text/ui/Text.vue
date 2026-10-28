<script setup lang="ts">
import type { TextProps } from "../data/types";

const { text } = defineProps<TextProps>();
</script>

<template>
    <div data-name="text">{text}</div>
</template>