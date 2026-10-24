<script setup lang="ts">
import Button from "../../../../../atoms/button/variant/default/ui/Button.vue";
import type { CardButtonProps } from "../data/types";

const { variant = "solid", ...restProps } = defineProps<CardButtonProps>();
</script>

<template>
    <Button :variant="variant" v-bind="restProps" />
</template>
