<script setup lang="ts">
import { computed, ref } from "vue";
import Button from "../../../../atoms/button/variant/default/ui/Button.vue";
import Icon from "../../../../atoms/icon/ui/Icon.vue";
import Image from "../../../../atoms/image/ui/Image.vue";
import SubmenuGridTitle from "../../shared/submenu/grid/title/ui/SubmenuGridTitle.vue";
import SubmenuSidebarGrid from "../../shared/submenu/grid/sidebar/ui/SubmenuSidebarGrid.vue";
import type { MenuProps } from "../data/types";
import { StyleComponent } from "../../../../../data/style.component";
import { getSubmenuLabel, isSidebarSubmenu, isSubmenu } from "../../../../../hooks/menu.hook";

const props = withDefaults(defineProps<MenuProps>(), {
  menus: () => [],
  buttonsProps: () => [],
  defaultOpenSubmenu: null,
});

const openSubmenu = ref<number | null>(props.defaultOpenSubmenu);
const active = computed(() =>
  openSubmenu.value === null ? undefined : props.menus[openSubmenu.value]
);
const nav = StyleComponent.menu.navigation;
</script>

<template>
  <nav :class="nav.root" aria-label="Menu principal">
    <div :class="nav.bar">
      <div :class="nav.logoContainer">
        <Image v-if="logo" v-bind="logo" :className="`${logo.className ?? ''} ${nav.logo}`" />
      </div>
      <ul :class="nav.list">
        <li v-for="(menu, index) in menus" :key="index" :class="nav.item">
          <button
            v-if="isSubmenu(menu)"
            type="button"
            :class="nav.submenuTrigger"
            :aria-expanded="openSubmenu === index"
            aria-haspopup="true"
            @keydown.esc="openSubmenu = null"
            @click="openSubmenu = openSubmenu === index ? null : index"
          >
            <span :class="nav.submenuLabel">{{ getSubmenuLabel(menu) }}</span>
            <Icon
              :name="openSubmenu === index ? 'ArrowUp' : 'ArrowDown'"
              :className="nav.submenuIcon"
            />
          </button>
          <a v-else-if="menu.href" :href="menu.href" :class="nav.link">
            <Icon v-if="menu.icon" v-bind="menu.icon" />
            <span v-if="menu.textProps" :class="nav.linkLabel">{{ menu.textProps.text }}</span>
          </a>
          <button v-else type="button" :class="nav.link">
            <Icon v-if="menu.icon" v-bind="menu.icon" />
            <span v-if="menu.textProps" :class="nav.linkLabel">{{ menu.textProps.text }}</span>
          </button>
        </li>
      </ul>
      <div :class="nav.actions">
        <Button v-for="(button, i) in buttonsProps" :key="i" v-bind="button" />
      </div>
    </div>
    <div v-if="active && isSubmenu(active)" :class="nav.panel">
      <SubmenuSidebarGrid v-if="isSidebarSubmenu(active)" v-bind="active" />
      <SubmenuGridTitle v-else v-bind="active" />
    </div>
  </nav>
</template>
