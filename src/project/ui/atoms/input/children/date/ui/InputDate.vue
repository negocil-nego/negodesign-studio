<script setup lang="ts">
import { computed } from "vue";
import Input from "../../../core/ui/Input.vue";
import InputLabel from "../../label/ui/InputLabel.vue";
import type { InputDateProps } from "../data/types";

const { labelProps, inputProps } = defineProps<InputDateProps>();
const componentProps = computed(() => ({ ...inputProps, type: "date" as const }));
</script>

<template>
  <InputLabel v-if="labelProps" :labelProps="labelProps" :inputProps="componentProps" />
  <Input v-else v-bind="componentProps" />
</template>
