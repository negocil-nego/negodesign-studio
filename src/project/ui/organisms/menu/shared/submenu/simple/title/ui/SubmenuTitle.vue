<script setup lang="ts">
import type { SubmenuTitleProps } from "../data/types";
import MenuText from "../../../../menu/menu-grid/ui/MenuText.vue";
import MenuTitle from "../../../../menu/menu-title/ui/MenuTitle.vue";
import { StyleComponent } from "../../../../../../../../../components/local/data/style.component";
defineProps<SubmenuTitleProps>();
</script>
<template><div :class="StyleComponent.menu.submenu.title.root"><MenuText v-if="title" v-bind="title" /><div v-if="items" :class="StyleComponent.menu.submenu.title.groups"><MenuTitle v-for="(item, i) in items" :key="i" v-bind="item" /></div></div></template>
