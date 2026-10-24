<script setup lang="ts">
import { useAttrs } from "vue";
import type { CardProps } from "../data/types";

defineProps<CardProps>();
const attrs = useAttrs();
</script>

<template>
    <div class="card" id="card">
        <div v-bind="attrs" />
    </div>
</template>
