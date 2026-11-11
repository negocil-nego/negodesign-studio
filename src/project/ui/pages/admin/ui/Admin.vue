<script setup lang="ts">
import AdminCrud from "./AdminCrud.vue";
import type { AdminProps } from "../data/types";

const props = defineProps<AdminProps>();
</script>

<template>
  <AdminCrud v-bind="props" />
</template>

