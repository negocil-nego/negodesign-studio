<script setup lang="ts">
import type { SubmenuGridTitleProps } from "../data/types";
import MenuTitle from "../../../../menu/menu-title/ui/MenuTitle.vue";
import SubmenuTitle from "../../../simple/title/ui/SubmenuTitle.vue";
import { StyleComponent } from "../../../../../../../../data/style.component";

defineProps<SubmenuGridTitleProps>();
const s = StyleComponent.menu.submenu.gridTitle;
</script>

<template>
  <div :class="s.root">
    <aside v-if="title" :class="s.intro">
      <MenuTitle v-bind="title" variant="featured" />
    </aside>
    <div v-if="items?.length" :class="s.groups">
      <div v-for="(item, i) in items" :key="i" :class="s.group">
        <SubmenuTitle v-bind="item" />
      </div>
    </div>
  </div>
</template>
