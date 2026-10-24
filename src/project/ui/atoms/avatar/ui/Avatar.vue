<script setup lang="ts">
import type { AvatarProps } from "../data/types";
import Skeleton from "../../skeleton/ui/Skeleton.vue";

defineProps<AvatarProps>();
</script>

<template>
    <Skeleton v-if="isLoading" :class-name="className || 'w-12 h-12 rounded-lg'" />
    <button v-else-if="src" :class="className" @click="onClick">
        <img :src="src" :alt="src" :class="className || 'w-12 h-12 rounded-lg'" />
    </button>
</template>
