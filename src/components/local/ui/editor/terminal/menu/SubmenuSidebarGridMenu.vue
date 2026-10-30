<script setup lang="ts">
import Menu from "../../../../../../project/ui/organisms/menu/simple/ui/Menu.vue";
import type { SubmenuSidebarGridProps } from "../../../../../../project/ui/organisms/menu/shared/submenu/grid/sidebar/data/types";
const solutionsMenu: SubmenuSidebarGridProps = {
  type: "sidebar-grid",
  title: { title: { text: "Soluções para cada etapa do seu projecto" }, description: { text: "Explore áreas de trabalho e encontre recursos para a sua equipa." }, href: "/main/project", link: { text: "Ver todas as soluções" } },
  defaultSelected: 0,
  items: [
    { title: { textProps: { text: "Design de produto" } }, items: [{ href: "/main/project", title: { text: "Biblioteca de componentes" }, description: { text: "Componentes reutilizáveis para interfaces consistentes." } }, { href: "/main/project", title: { text: "Sistemas de design" }, description: { text: "Organize padrões e tokens para toda a equipa." } }] },
    { title: { textProps: { text: "Desenvolvimento" } }, items: [{ href: "/main/project", title: { text: "Aplicações web" }, description: { text: "Desenvolva produtos modernos e adaptáveis." } }, { href: "/main/project", title: { text: "Integrações" }, description: { text: "Ligue ferramentas e serviços do seu projecto." } }] },
  ],
};
const menus = [{ href: "/", icon: { name: "Home" }, textProps: { text: "Início" } }, solutionsMenu, { href: "/main/project", textProps: { text: "Projectos" } }];
</script>
<template><Menu :logo="{ src: '/img/programmer-02.jpg', alt: 'Negodesign' }" :menus="menus" :buttonsProps="[{ text: 'Entrar', variant: 'outline' }, { text: 'Começar', variant: 'solid' }]" :defaultOpenSubmenu="1" /></template>
