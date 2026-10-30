<script setup lang="ts">
import { computed, ref } from "vue";
import Button from "../../../../atoms/button/variant/default/ui/Button.vue";
import Icon from "../../../../atoms/icon/ui/Icon.vue";
import Image from "../../../../atoms/image/ui/Image.vue";
import SubmenuGridTitle from "../../shared/submenu/grid/title/ui/SubmenuGridTitle.vue";
import SubmenuSidebarGrid from "../../shared/submenu/grid/sidebar/ui/SubmenuSidebarGrid.vue";
import type { MenuProps } from "../data/types";
import { StyleComponent } from "../../../../../data/style.component";
const props = withDefaults(defineProps<MenuProps>(), { menus: () => [], buttonsProps: () => [], defaultOpenSubmenu: null });
const openSubmenu = ref<number | null>(props.defaultOpenSubmenu);
const active = computed(() => openSubmenu.value === null ? undefined : props.menus[openSubmenu.value]);
const isSidebar = (menu: any) => menu?.type === "sidebar-grid";
const isSubmenu = (menu: any) => !!menu && (isSidebar(menu) || "items" in menu || "title" in menu || "label" in menu);
const menuLabel = (menu: any) => (isSidebar(menu) ? menu.title?.title?.text : menu.label ?? menu.title?.title?.text) ?? "Explorar";
</script>
<template><nav :class="StyleComponent.menu.navigation.root" aria-label="Menu principal"><div :class="StyleComponent.menu.navigation.bar"><div :class="StyleComponent.menu.navigation.logoContainer"><Image v-if="logo" v-bind="logo" :className="`${logo.className ?? ''} ${StyleComponent.menu.navigation.logo}`" /></div><ul :class="StyleComponent.menu.navigation.list"><li v-for="(menu, index) in menus" :key="index" :class="StyleComponent.menu.navigation.item"><button v-if="isSubmenu(menu)" type="button" :class="StyleComponent.menu.navigation.submenuTrigger" :aria-expanded="openSubmenu === index" aria-haspopup="true" @keydown.esc="openSubmenu = null" @click="openSubmenu = openSubmenu === index ? null : index"><span :class="StyleComponent.menu.navigation.submenuLabel">{{ menuLabel(menu) }}</span><Icon :name="openSubmenu === index ? 'ArrowUp' : 'ArrowDown'" :className="StyleComponent.menu.navigation.submenuIcon" /></button><a v-else-if="menu.href" :href="menu.href" :class="StyleComponent.menu.navigation.link"><Icon v-if="menu.icon" v-bind="menu.icon" /><span v-if="menu.textProps" :class="StyleComponent.menu.navigation.linkLabel">{{ menu.textProps.text }}</span></a><button v-else type="button" :class="StyleComponent.menu.navigation.link"><Icon v-if="menu.icon" v-bind="menu.icon" /><span v-if="menu.textProps" :class="StyleComponent.menu.navigation.linkLabel">{{ menu.textProps.text }}</span></button></li></ul><div :class="StyleComponent.menu.navigation.actions"><Button v-for="(button, i) in buttonsProps" :key="i" v-bind="button" /></div></div><div v-if="isSubmenu(active)" :class="StyleComponent.menu.navigation.panel"><SubmenuSidebarGrid v-if="isSidebar(active)" v-bind="active" /><SubmenuGridTitle v-else v-bind="active" /></div></nav></template>
