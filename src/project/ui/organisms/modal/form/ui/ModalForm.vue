<script setup lang="ts">
import type { FormProps } from "../../../../molecules/form/data/types";
import Modal from "../../core/ui/Modal.vue";
import Form from "../../../../molecules/form/ui/form.vue";
defineProps<{ open?: boolean; title?: string; form?: FormProps }>();
const emit = defineEmits<{ close: [] }>();
</script>
<template><Modal :open="open" :title="title" @close="emit('close')"><Form v-if="form" v-bind="form" /></Modal></template>
