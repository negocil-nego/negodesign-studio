<script setup lang="ts">
import Avatar from "../../../../../atoms/avatar/ui/Avatar.vue";
import type { CardLogoProps } from "../data/types";

const { ...restProps } = defineProps<CardLogoProps>();
</script>

<template>
    <Avatar v-bind="restProps" />
</template>
