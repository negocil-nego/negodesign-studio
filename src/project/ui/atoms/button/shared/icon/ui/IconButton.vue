<script setup lang="ts">
import Button from "../../../variant/default/ui/Button.vue";
import type { IconButtonProps } from "../data/types";

withDefaults(defineProps<IconButtonProps>(), { isLoading: false });
</script>

<template>
    <Button
        class-name="w-10 h-10 rounded-full border border-gray-100 flex items-center justify-center cursor-pointer bg-gray-100"
        :icon="icon"
        :is-skeleton="isLoading"
        @click="onClick"
    />
</template>
