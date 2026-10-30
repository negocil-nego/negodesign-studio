<script setup lang="ts">
import DualSection from "../../../../templates/section/dual-section/ui/DualSection.vue";
const fields = [
  { component: "input-email" as const, labelProps: { label: "Email", for: "login-dual-email" }, inputProps: { id: "login-dual-email", name: "email", type: "email" as const, placeholder: "nome@exemplo.com", required: true } },
  { component: "input-password" as const, labelProps: { label: "Palavra-passe", for: "login-dual-password" }, inputProps: { id: "login-dual-password", name: "password", type: "password" as const, placeholder: "A sua palavra-passe", required: true } },
];
const leftSectionForm = { isScreen: true, className: "flex items-center justify-center bg-blue-600 px-8 py-10 text-white", titleProps: { text: "Negodesign" }, subtitleProps: { text: "Crie com confiança." }, descriptionProps: { text: "Os seus projectos e ferramentas num só lugar." } };
const rightSectionForm = { isScreen: true, className: "flex items-center justify-center bg-blender-card px-6 py-8 text-zinc-100", titleProps: { text: "Iniciar sessão" }, subtitleProps: { text: "Bem-vindo de volta." }, formProps: { style: "inline" as const, fields }, buttonSubmitProps: { text: "Entrar" }, buttonsLoginProps: [{ text: "Não tem conta? Registe-se", variant: "outline" as const }] };
</script>
<template><DualSection :leftSectionForm="leftSectionForm" :rightSectionForm="rightSectionForm" /></template>
