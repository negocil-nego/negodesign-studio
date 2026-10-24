<script setup lang="ts">
import ButtonBase from "../../../shared/base/ui/ButtonBase.vue";
import type { OutlineButtonProps } from "../data/types";

const props = defineProps<OutlineButtonProps>();
</script>

<template>
    <ButtonBase class-name="w-full py-2 px-4 rounded-md text-center border border-primary/40 text-primary-foreground" v-bind="props" />
</template>
