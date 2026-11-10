<script setup lang="ts">
import { computed, ref } from "vue";
import type { SubmenuSidebarGridProps } from "../data/types";
import MenuTitle from "../../../../menu/menu-title/ui/MenuTitle.vue";
import SubmenuTitle from "../../../simple/title/ui/SubmenuTitle.vue";
import { StyleComponent } from "../../../../../../../../data/style.component";

const props = withDefaults(defineProps<SubmenuSidebarGridProps>(), {
  items: () => [],
  defaultSelected: 0,
});

const selected = ref(Math.min(props.defaultSelected, Math.max(props.items.length - 1, 0)));
const active = computed(() => props.items[selected.value]);
const s = StyleComponent.menu.submenu.sidebarGrid;
</script>

<template>
  <div :class="s.root">
    <aside :class="s.sidebar">
      <p :class="s.sidebarHeader">Navegação</p>
      <nav :class="s.nav" aria-label="Secções do submenu">
        <button
          v-for="(item, i) in items"
          :key="i"
          type="button"
          :class="`${s.navItem} ${selected === i ? s.navItemActive : ''}`"
          :aria-current="selected === i ? 'page' : undefined"
          @click="selected = i"
        >
          {{ item.title?.textProps?.text ?? `Secção ${i + 1}` }}
        </button>
      </nav>
    </aside>
    <section :class="s.content">
      <div :class="s.contentInner">
        <header v-if="title" :class="s.contentHeader">
          <MenuTitle v-bind="title" variant="featured" />
        </header>
        <div v-if="active" :class="s.groupContent">
          <SubmenuTitle v-bind="active" />
        </div>
        <div v-else :class="s.empty">Sem secções para apresentar.</div>
      </div>
    </section>
  </div>
</template>
