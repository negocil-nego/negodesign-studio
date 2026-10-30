<script setup lang="ts">
import DualSection from "../../../../templates/section/dual-section/ui/DualSection.vue";
const fields = [
  { component: "input-text" as const, labelProps: { label: "Nome", for: "register-dual-name" }, inputProps: { id: "register-dual-name", name: "name", type: "text" as const, placeholder: "O seu nome", required: true } },
  { component: "input-email" as const, labelProps: { label: "Email", for: "register-dual-email" }, inputProps: { id: "register-dual-email", name: "email", type: "email" as const, placeholder: "nome@exemplo.com", required: true } },
  { component: "input-password" as const, labelProps: { label: "Palavra-passe", for: "register-dual-password" }, inputProps: { id: "register-dual-password", name: "password", type: "password" as const, placeholder: "Crie uma palavra-passe", required: true } },
];
const leftSectionForm = { isScreen: true, className: "flex items-center justify-center bg-blue-600 px-8 py-10 text-white", titleProps: { text: "Negodesign" }, subtitleProps: { text: "O seu próximo projecto começa aqui." }, descriptionProps: { text: "Junte-se à comunidade e transforme ideias em interfaces." } };
const rightSectionForm = { isScreen: true, className: "flex items-center justify-center bg-blender-card px-6 py-8 text-zinc-100", titleProps: { text: "Criar conta" }, subtitleProps: { text: "É rápido e gratuito." }, formProps: { style: "inline" as const, fields }, buttonSubmitProps: { text: "Criar conta" }, buttonsLoginProps: [{ text: "Já tem conta? Iniciar sessão", variant: "outline" as const }] };
</script>
<template><DualSection :leftSectionForm="leftSectionForm" :rightSectionForm="rightSectionForm" /></template>
