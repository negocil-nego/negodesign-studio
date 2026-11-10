<script setup lang="ts">
import { computed, ref } from "vue";
import Icon from "../../../atoms/icon/ui/Icon.vue";
import SidebarProfile from "../shared/sidebar-profile/ui/SidebarProfile.vue";
import SidebarSection from "../shared/sidebar-section/ui/SidebarSection.vue";
import {
  computeSidebarTheme,
  SIDEBAR_RAIL_FOOTER_ICONS,
  SIDEBAR_RAIL_ICONS,
} from "../../../../hooks/sidebar.hook";
import type { SidebarProps } from "../data/types";

const props = withDefaults(defineProps<SidebarProps>(), {
  variant: "01",
  theme: "light",
  collapsed: false,
  logo: "◈",
  brand: "Untitled UI",
  workspace: "workspace.untitledui.com",
  sections: () => [],
  footerSections: () => [],
  user: () => ({ name: "Sienna Hewitt", detail: "sienna@untitledui.com" }),
  className: "",
});

const cfg = computed(() => computeSidebarTheme(props.variant, props.theme, props.collapsed, props.className));
const query = ref("");
</script>

<template>
  <aside :aria-label="`${brand} sidebar`" :class="cfg.rootClass">
    <nav v-if="cfg.rail" aria-label="Shortcuts" :class="cfg.railClass">
      <b class="grid h-9 w-9 place-items-center rounded-xl bg-blue-600 text-white">∞</b>
      <span class="my-2 h-8 w-8 rounded-full bg-blue-100 ring-2 ring-blue-500"></span>
      <button v-for="(icon, i) in SIDEBAR_RAIL_ICONS" :key="icon" :aria-label="icon"
        :class="`grid h-9 w-9 place-items-center rounded-lg ${i === 3 ? 'bg-blue-600 text-white' : cfg.dark ? 'text-zinc-400 hover:bg-zinc-800' : 'text-zinc-500 hover:bg-zinc-200'}`">
        <Icon :name="icon" />
      </button>
      <div class="flex-1"></div>
      <button v-for="icon in SIDEBAR_RAIL_FOOTER_ICONS" :key="icon" :aria-label="icon" class="grid h-8 w-8 place-items-center rounded-lg text-zinc-500">
        <Icon :name="icon" />
      </button>
    </nav>

    <div class="flex min-w-0 flex-1 flex-col p-4">
      <header class="mb-4 flex min-h-9 items-center gap-2">
        <span v-if="!cfg.rail" :class="cfg.logoBoxClass">{{ logo }}</span>
        <div v-if="!cfg.compact" class="min-w-0 flex-1">
          <p class="truncate text-sm font-bold">{{ brand }}</p>
          <p v-if="variant === '04'" class="truncate text-[10px] text-zinc-500">{{ workspace }}</p>
        </div>
        <button v-if="variant === '04' && !cfg.compact" aria-label="Switch workspace">⌄</button>
      </header>

      <label v-if="cfg.showSearch" :class="cfg.searchClass">
        <Icon name="Search" className="text-zinc-400" />
        <input v-model="query" :placeholder="cfg.compact ? '' : 'Search'" aria-label="Search"
          class="w-full min-w-0 bg-transparent text-xs outline-none placeholder:text-zinc-400" />
        <kbd v-if="!cfg.compact" class="rounded border px-1 text-[9px] text-zinc-400">⌘ K</kbd>
      </label>

      <div class="min-h-0 flex-1 space-y-3 overflow-y-auto">
        <SidebarSection v-for="(section, i) in sections" :key="`${section.title}-${i}`"
          :section="section" :compact="cfg.compact" :dark="cfg.dark" :accent="cfg.accent" />
      </div>

      <div v-if="storage && !cfg.compact" :class="cfg.storageClass">
        <p class="text-xs font-semibold">{{ storage.title }}</p>
        <p class="mt-1 text-[10px] leading-4 text-zinc-500">{{ storage.description }}</p>
        <div class="mt-3 h-1.5 overflow-hidden rounded-full bg-zinc-200">
          <div class="h-full rounded-full bg-blue-600" :style="{ width: `${Math.min(100, Math.max(0, storage.usage))}%` }"></div>
        </div>
      </div>

      <div v-if="footerSections.length" class="space-y-1 border-t border-current/10 py-3">
        <SidebarSection v-for="(section, i) in footerSections" :key="`f-${i}`"
          :section="section" :compact="cfg.compact" :dark="cfg.dark" />
      </div>

      <SidebarProfile :user="user" :compact="cfg.compact" :dark="cfg.dark" />
    </div>
  </aside>
</template>
