<script setup lang="ts">
import { ref } from "vue";
import Button from "../../../../../../project/ui/atoms/button/variant/default/ui/Button.vue";
import ModalForm from "../../../../../../project/ui/organisms/modal/form/ui/ModalForm.vue";
import type { FormProps } from "../../../../../../project/ui/molecules/form/data/types";
const open = ref(false);
const form: FormProps = {
  style: "grid",
  fields: [
    { component: "input-text", id: "project-name", labelProps: { label: "Nome do projecto", for: "project-name" }, inputProps: { id: "project-name", name: "projectName", type: "text", placeholder: "O seu projecto" } },
    { component: "input-email", id: "contact-email", labelProps: { label: "Email de contacto", for: "contact-email" }, inputProps: { id: "contact-email", name: "email", type: "email", placeholder: "nome@exemplo.com" } },
  ],
};
</script>
<template>
  <div class="flex min-h-48 items-center justify-center rounded-lg border border-blender-border bg-blender-base p-6">
    <Button text="Abrir formulário" @click="open = true" />
    <ModalForm :open="open" title="Novo projecto" :form="form" @close="open = false" />
  </div>
</template>
