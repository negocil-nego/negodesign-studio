<script setup lang="ts">
import Menu from "../../../../../../project/ui/organisms/menu/simple/ui/Menu.vue";
import type { SubmenuGridTitleProps } from "../../../../../../project/ui/organisms/menu/shared/submenu/grid/title/data/types";
const platformMenu: SubmenuGridTitleProps = {
  label: "Plataforma",
  title: { title: { text: "Uma plataforma para criar com confiança" }, description: { text: "Organize projectos, componentes e sistemas num só espaço." }, href: "/main/project", link: { text: "Saiba mais" } },
  items: [
    { title: { textProps: { text: "PRINCIPAIS RECURSOS" } }, items: [{ href: "/main/project", title: { text: "Biblioteca de componentes" }, description: { text: "Construa interfaces consistentes com componentes reutilizáveis." } }, { href: "/main/project", title: { text: "Editor visual" }, description: { text: "Explore e ajuste componentes num espaço interactivo." } }] },
    { title: { textProps: { text: "DESCOBRIR" } }, items: [{ href: "/main/project", title: { text: "Documentação" }, description: { text: "Consulte padrões, propriedades e exemplos." } }, { href: "/main/project", title: { text: "Actualizações" }, description: { text: "Veja as novidades mais recentes." } }] },
  ],
};
const menus = [{ href: "/", icon: { name: "Home" }, textProps: { text: "Início" } }, platformMenu, { href: "/main/project", textProps: { text: "Equipa" } }];
</script>
<template><Menu :logo="{ src: '/img/programmer-01.jpg', alt: 'Negodesign', className: 'h-9 w-9 rounded-md object-cover' }" :menus="menus" :buttonsProps="[{ text: 'Entrar', variant: 'outline' }, { text: 'Criar conta', variant: 'solid' }]" :defaultOpenSubmenu="1" /></template>
