<script setup lang="ts">
import SidebarItem from "../../sidebar-item/ui/SidebarItem.vue";
import type { SidebarSectionProps } from "../../../data/types";
withDefaults(
  defineProps<{
    section: SidebarSectionProps;
    compact?: boolean;
    dark?: boolean;
    accent?: string;
  }>(),
  { compact: false, dark: false, accent: "blue" },
);
</script>
<template>
  <section class="space-y-1.5">
    <h3
      v-if="section.title && !compact"
      :class="`px-3 pb-1 pt-3 text-[10px] font-semibold uppercase tracking-[0.14em] ${dark ? 'text-zinc-500' : 'text-zinc-400'}`"
    >
      {{ section.title }}
    </h3>
    <div class="space-y-1">
      <SidebarItem
        v-for="item in section.items"
        :key="item.label"
        :item="item"
        :compact="compact"
        :dark="dark"
        :accent="accent"
      />
    </div>
  </section>
</template>
