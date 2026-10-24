<script setup lang="ts">
import { computed } from "vue";
import type { CardProductProps } from "../data/types";
import CardProduct01 from "./card-01/CardProduct01.vue";
import CardProduct02 from "./card-02/CardProduct02.vue";

const props = withDefaults(defineProps<CardProductProps>(), { variant: "01" });

const restProps = computed(() => {
    const { variant, ...rest } = props;
    return rest;
});
</script>

<template>
    <CardProduct01 v-if="props.variant === '01'" v-bind="restProps" />
    <CardProduct02 v-else-if="props.variant === '02'" v-bind="restProps" />
</template>
