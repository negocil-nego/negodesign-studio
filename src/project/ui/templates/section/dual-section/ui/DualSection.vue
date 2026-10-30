<script setup lang="ts">
import type { LoginDualSectionProps } from "../data/types";
import SectionForm from "../../../../organisms/section/form/ui/SectionForm.vue";
defineProps<LoginDualSectionProps>();
</script>
<template><section class="grid h-full min-h-full grid-cols-1 md:grid-cols-2"><SectionForm v-bind="leftSectionForm" /><SectionForm v-bind="rightSectionForm" /></section></template>
