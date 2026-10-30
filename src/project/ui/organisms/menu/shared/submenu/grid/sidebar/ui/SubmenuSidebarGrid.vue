<script setup lang="ts">
import { computed, ref } from "vue";
import type { SubmenuSidebarGridProps } from "../data/types";
import MenuTitle from "../../../../menu/menu-title/ui/MenuTitle.vue";
import SubmenuTitle from "../../../simple/title/ui/SubmenuTitle.vue";
import { StyleComponent } from "../../../../../../../../../components/local/data/style.component";
const props = withDefaults(defineProps<SubmenuSidebarGridProps>(), { items: () => [], defaultSelected: 0 });
const selected = ref(Math.min(props.defaultSelected, Math.max(props.items.length - 1, 0)));
const active = computed(() => props.items[selected.value]);
</script>
<template><div :class="StyleComponent.menu.submenu.sidebarGrid.root"><aside :class="StyleComponent.menu.submenu.sidebarGrid.sidebar"><p :class="StyleComponent.menu.submenu.sidebarGrid.sidebarHeader">Navegação</p><nav :class="StyleComponent.menu.submenu.sidebarGrid.nav" aria-label="Secções do submenu"><button v-for="(item, i) in items" :key="i" type="button" :class="`${StyleComponent.menu.submenu.sidebarGrid.navItem} ${selected === i ? StyleComponent.menu.submenu.sidebarGrid.navItemActive : ''}`" :aria-current="selected === i ? 'page' : undefined" @click="selected = i">{{ item.title?.textProps?.text ?? `Secção ${i + 1}` }}</button></nav></aside><section :class="StyleComponent.menu.submenu.sidebarGrid.content"><div :class="StyleComponent.menu.submenu.sidebarGrid.contentInner"><header v-if="title" :class="StyleComponent.menu.submenu.sidebarGrid.contentHeader"><MenuTitle v-bind="title" variant="featured" /></header><div v-if="active" :class="StyleComponent.menu.submenu.sidebarGrid.groupContent"><SubmenuTitle v-bind="active" /></div><div v-else :class="StyleComponent.menu.submenu.sidebarGrid.empty">Sem secções para apresentar.</div></div></section></div></template>
