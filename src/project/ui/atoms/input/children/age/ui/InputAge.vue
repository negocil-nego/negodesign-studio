<script setup lang="ts">
import { computed } from "vue";
import Input from "../../../core/ui/Input.vue";
import InputLabel from "../../label/ui/InputLabel.vue";
import type { InputAgeProps } from "../data/types";

const { labelProps, inputProps } = defineProps<InputAgeProps>();
const componentProps = computed(() => ({ ...inputProps, type: "number" as const }));
</script>

<template>
  <InputLabel v-if="labelProps" :labelProps="labelProps" :inputProps="componentProps" />
  <Input v-else v-bind="componentProps" />
</template>
