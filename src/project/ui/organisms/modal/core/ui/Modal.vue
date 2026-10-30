<script setup lang="ts">
import { nextTick, onMounted, ref, watch } from "vue";
import { StyleComponent } from "../../../../../data/style.component";
const props = withDefaults(defineProps<{ open?: boolean; title?: string }>(), { open: false });
const emit = defineEmits<{ close: [] }>();
const dialog = ref<HTMLDialogElement>();
function sync() { const el = dialog.value; if (!el) return; if (props.open && !el.open) el.showModal(); else if (!props.open && el.open) el.close(); }
function close() { emit("close"); }
onMounted(sync); watch(() => props.open, async () => { await nextTick(); sync(); });
</script>
<template><dialog ref="dialog" :class="StyleComponent.modal.root" :aria-label="title" aria-modal="true" @close="close" @click="(event) => { if (event.target === dialog) dialog?.close(); }"><slot /></dialog></template>
