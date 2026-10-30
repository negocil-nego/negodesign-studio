<script setup lang="ts">
import type { SimpleProps } from "../data/types";
import SectionForm from "../../../../organisms/section/form/ui/SectionForm.vue";
defineProps<SimpleProps>();
</script>
<template><section class="h-full min-h-full"><SectionForm v-bind="sectionFormProps" /></section></template>
