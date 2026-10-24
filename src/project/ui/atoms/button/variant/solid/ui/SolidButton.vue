<script setup lang="ts">
import ButtonBase from "../../../shared/base/ui/ButtonBase.vue";
import type { SolidButtonProps } from "../data/types";

const props = defineProps<SolidButtonProps>();
</script>

<template>
    <ButtonBase class-name="w-full py-2 px-4 rounded-md text-center bg-blue-900 hover:bg-primary/80 text-white" v-bind="props" />
</template>
