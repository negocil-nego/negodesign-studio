<script setup lang="ts">
import InputUsername from "../../../ui/InputUsername.vue";
import type { InputUsernameOrEmailProps } from "../data/types";

const props = defineProps<InputUsernameOrEmailProps>();
</script>

<template>
  <InputUsername v-bind="props" :inputProps="{ ...props.inputProps, icon: props.inputProps.icon ?? 'Email' }" validationMode="username-or-email" />
</template>
