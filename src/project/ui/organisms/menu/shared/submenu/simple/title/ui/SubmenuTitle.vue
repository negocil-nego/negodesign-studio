<script setup lang="ts">
import type { SubmenuTitleProps } from "../data/types";
import MenuText from "../../../../menu/menu-grid/ui/MenuText.vue";
import MenuTitle from "../../../../menu/menu-title/ui/MenuTitle.vue";
import { StyleComponent } from "../../../../../../../../data/style.component";

defineProps<SubmenuTitleProps>();
const s = StyleComponent.menu.submenu.title;
</script>

<template>
  <div :class="s.root">
    <MenuText v-if="title" v-bind="title" />
    <div v-if="items" :class="s.groups">
      <MenuTitle v-for="(item, i) in items" :key="i" v-bind="item" />
    </div>
  </div>
</template>
