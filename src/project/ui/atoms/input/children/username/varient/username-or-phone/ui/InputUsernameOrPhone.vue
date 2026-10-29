<script setup lang="ts">
import InputUsername from "../../../ui/InputUsername.vue";
import type { InputUsernameOrPhoneProps } from "../data/types";

const props = defineProps<InputUsernameOrPhoneProps>();
</script>

<template>
  <InputUsername v-bind="props" validationMode="username-or-phone" />
</template>
