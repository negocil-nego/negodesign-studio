<script setup lang="ts">
import type { SubmenuGridTitleProps } from "../data/types";
import MenuTitle from "../../../../menu/menu-title/ui/MenuTitle.vue";
import SubmenuTitle from "../../../simple/title/ui/SubmenuTitle.vue";
import { StyleComponent } from "../../../../../../../../../components/local/data/style.component";
defineProps<SubmenuGridTitleProps>();
</script>
<template><div :class="StyleComponent.menu.submenu.gridTitle.root"><aside v-if="title" :class="StyleComponent.menu.submenu.gridTitle.intro"><MenuTitle v-bind="title" variant="featured" /></aside><div v-if="items?.length" :class="StyleComponent.menu.submenu.gridTitle.groups"><div v-for="(item, i) in items" :key="i" :class="StyleComponent.menu.submenu.gridTitle.group"><SubmenuTitle v-bind="item" /></div></div></div></template>
