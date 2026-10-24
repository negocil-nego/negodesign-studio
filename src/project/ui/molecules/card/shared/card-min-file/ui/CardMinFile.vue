<script setup lang="ts">
import type { CardMinFileProps } from "../data/types";
import CardImage from "../../card-image/ui/CardImage.vue";

const { imageProps, active = false, onClick } = defineProps<CardMinFileProps>();
</script>

<template>
    <button
        type="button"
        :class="['relative shrink-0 size-14 overflow-hidden rounded-md border border-blender-border cursor-pointer transition-colors [&>img]:h-full [&>img]:w-full [&>img]:rounded-none [&>img]:object-cover [&>img]:transition-none', { 'border-blue-500 ring-1 ring-blue-500': active }]"
        @click="onClick?.()"
    >
        <CardImage v-if="imageProps" v-bind="imageProps" />
    </button>
</template>
