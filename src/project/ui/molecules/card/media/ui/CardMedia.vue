<script setup lang="ts">
import { computed } from "vue";
import type { CardMediaProps } from "../data/types";
import CardMedia01 from "./card-01/CardMedia01.vue";
import CardMedia02 from "./card-02/CardMedia02.vue";

const props = withDefaults(defineProps<CardMediaProps>(), { variant: "01" });

const restProps = computed(() => {
    const { variant, ...rest } = props;
    return rest;
});
</script>

<template>
    <CardMedia01 v-if="props.variant === '01'" v-bind="restProps" />
    <CardMedia02 v-else-if="props.variant === '02'" v-bind="restProps" />
</template>
