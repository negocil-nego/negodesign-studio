<script setup lang="ts">
import type { SkeletonProps } from "../data/types";
import { StyleComponent } from "../../../../data/style.component";

withDefaults(defineProps<SkeletonProps>(), { className: "" });
</script>

<template>
    <div
        aria-hidden="true"
        :class="`${StyleComponent.skeleton.default} ${className}`"
    ></div>
</template>
