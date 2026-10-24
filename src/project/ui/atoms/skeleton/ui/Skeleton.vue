<script setup lang="ts">
import type { SkeletonProps } from "../data/types";

withDefaults(defineProps<SkeletonProps>(), { className: "" });
</script>

<template>
    <div
        aria-hidden="true"
        :class="`relative overflow-hidden rounded-xl bg-gray-200 before:absolute before:inset-0 before:-translate-x-full before:animate-shimmer before:bg-linear-to-r before:from-transparent before:via-white/70 before:to-transparent ${className}`"
    ></div>
</template>
