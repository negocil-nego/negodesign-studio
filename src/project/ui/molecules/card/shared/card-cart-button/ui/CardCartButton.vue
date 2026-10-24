<script setup lang="ts">
import IconButton from "../../../../../atoms/button/shared/icon/ui/IconButton.vue";
import type { CardCartButtonProps } from "../data/types";

const { ...restProps } = defineProps<CardCartButtonProps>();
</script>

<template>
    <IconButton :icon="{ name: 'Cart' }" v-bind="restProps" />
</template>
