<script setup lang="ts">
import { computed } from "vue";
import type { CardProfileProps } from "../data/types";
import CardProfile01 from "./card-01/CardProfile01.vue";
import CardProfile02 from "./card-02/CardProfile02.vue";

const props = withDefaults(defineProps<CardProfileProps>(), { variant: "01" });

const restProps = computed(() => {
    const { variant, ...rest } = props;
    return rest;
});
</script>

<template>
    <CardProfile01 v-if="props.variant === '01'" v-bind="restProps" />
    <CardProfile02 v-else-if="props.variant === '02'" v-bind="restProps" />
</template>
