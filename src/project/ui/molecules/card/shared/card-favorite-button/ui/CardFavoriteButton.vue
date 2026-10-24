<script setup lang="ts">
import IconButton from "../../../../../atoms/button/shared/icon/ui/IconButton.vue";
import type { CardFavoriteButtonProps } from "../data/types";

const { ...restProps } = defineProps<CardFavoriteButtonProps>();
</script>

<template>
    <IconButton :icon="{ name: 'Favorite' }" v-bind="restProps" />
</template>
