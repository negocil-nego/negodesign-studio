<script setup lang="ts">
import type { ImageProps } from "../data/types.ts";
import Skeleton from "../../skeleton/ui/Skeleton.vue";
import { StyleComponent } from "../../../../data/style.component.ts";

defineProps<ImageProps>();
</script>

<template>
    <Skeleton v-if="isLoading" :class-name="className || StyleComponent.image.default" />
    <button v-else-if="src" :class="className" @click="onClick">
        <img :src="src" :alt="src" :class="className || StyleComponent.image.default" />
    </button>
</template>
