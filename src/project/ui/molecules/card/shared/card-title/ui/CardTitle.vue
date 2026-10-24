<script setup lang="ts">
import type { CardTitleProps } from "../data/types";

const { title, className } = defineProps<CardTitleProps>();
</script>

<template>
    <div :class="['card-title', className]" id="card-title">
        <div class="card-title-text">{{ title }}</div>
    </div>
</template>
