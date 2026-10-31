<script setup lang="ts">
import InputUsername from "../../../ui/InputUsername.vue";
import type { InputUsernameOrPhoneProps } from "../data/types";

const props = defineProps<InputUsernameOrPhoneProps>();
</script>

<template>
  <InputUsername v-bind="props" :inputProps="{ ...props.inputProps, icon: props.inputProps.icon ?? 'Phone' }" validationMode="username-or-phone" />
</template>
