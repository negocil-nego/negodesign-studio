<script setup lang="ts">
import { ref } from "vue";
import Icon from "../../../atoms/icon/ui/Icon.vue";
import SidebarSection from "../shared/sidebar-section/ui/SidebarSection.vue";
import SidebarProfile from "../shared/sidebar-profile/ui/SidebarProfile.vue";
import type { SidebarProps } from "../data/types";
withDefaults(defineProps<SidebarProps>(), {
  variant: "01",
  theme: "light",
  collapsed: false,
  logo: "◈",
  brand: "Untitled UI",
  workspace: "workspace.untitledui.com",
  sections: () => [],
  footerSections: () => [],
  user: () => ({ name: "Sienna Hewitt", detail: "sienna@untitledui.com" }),
});
const query = ref("");
</script>
<template>
  <aside
    :aria-label="`${brand} sidebar`"
    :class="`flex h-[min(760px,90vh)] min-h-[560px] shrink-0 overflow-hidden rounded-2xl border shadow-sm ${theme === 'dark' ? 'border-zinc-800 bg-zinc-950 text-zinc-100' : 'border-zinc-200 bg-white text-zinc-900'} ${variant === '02' ? 'w-[min(100%,420px)] flex-row' : collapsed ? 'w-[76px]' : 'w-[min(100%,280px)]'} ${className ?? ''}`"
  >
    <nav
      v-if="variant === '02'"
      aria-label="Atalhos"
      :class="`flex w-[68px] shrink-0 flex-col items-center gap-3 border-r p-3 ${theme === 'dark' ? 'border-zinc-800 bg-zinc-900' : 'border-zinc-100 bg-zinc-50'}`"
    >
      <b
        class="grid h-9 w-9 place-items-center rounded-xl bg-blue-600 text-white"
        >∞</b
      ><span
        class="my-2 h-8 w-8 rounded-full bg-blue-100 ring-2 ring-blue-500"
      /><button
        v-for="(name, i) in [
          'Grid',
          'Search',
          'Star',
          'Home',
          'Mail',
          'User',
          'Settings',
        ]"
        :key="name"
        :aria-label="name"
        :class="`grid h-9 w-9 place-items-center rounded-lg ${i === 3 ? 'bg-blue-600 text-white' : theme === 'dark' ? 'text-zinc-400 hover:bg-zinc-800' : 'text-zinc-500 hover:bg-zinc-200'}`"
      >
        <Icon :name="name" />
      </button>
      <div class="flex-1" />
      <Icon name="Settings" />
    </nav>
    <div class="flex min-w-0 flex-1 flex-col p-4">
      <header class="mb-4 flex min-h-9 items-center gap-2">
        <span
          v-if="variant !== '02'"
          :class="`grid h-8 w-8 shrink-0 place-items-center rounded-lg text-lg font-bold ${theme === 'dark' ? 'bg-zinc-800' : 'bg-zinc-100'}`"
          >{{ logo }}</span
        >
        <div v-if="!collapsed" class="min-w-0 flex-1">
          <p class="truncate text-sm font-bold">{{ brand }}</p>
          <p v-if="variant === '04'" class="truncate text-[10px] text-zinc-500">
            {{ workspace }}
          </p>
        </div>
        <button
          v-if="variant === '04' && !collapsed"
          aria-label="Mudar espaço de trabalho"
        >
          ⌄
        </button>
      </header>
      <label
        v-if="variant === '01' || variant === '02'"
        :class="`mb-4 flex h-9 items-center gap-2 rounded-lg border px-2.5 ${theme === 'dark' ? 'border-zinc-700 bg-zinc-900' : 'border-zinc-200 bg-white'}`"
        ><Icon name="Search" className="text-zinc-400" /><input
          v-model="query"
          aria-label="Pesquisar"
          :placeholder="collapsed ? '' : 'Pesquisar'"
          class="w-full min-w-0 bg-transparent text-xs outline-none"
        /><kbd
          v-if="!collapsed"
          class="rounded border px-1 text-[9px] text-zinc-400"
          >⌘ K</kbd
        ></label
      >
      <div class="min-h-0 flex-1 space-y-3 overflow-y-auto">
        <SidebarSection
          v-for="(section, i) in sections"
          :key="i"
          :section="section"
          :compact="collapsed"
          :dark="theme === 'dark'"
          :accent="variant === '03' ? 'violet' : 'blue'"
        />
      </div>
      <div v-if="storage && !collapsed" class="my-3 rounded-xl border p-3">
        <p class="text-xs font-semibold">{{ storage.title }}</p>
        <p class="mt-1 text-[10px] text-zinc-500">{{ storage.description }}</p>
        <progress class="mt-3 w-full" max="100" :value="storage.usage" />
      </div>
      <div
        v-if="footerSections.length"
        class="space-y-1 border-t border-current/10 py-3"
      >
        <SidebarSection
          v-for="(section, i) in footerSections"
          :key="i"
          :section="section"
          :compact="collapsed"
          :dark="theme === 'dark'"
        />
      </div>
      <SidebarProfile
        :user="user"
        :compact="collapsed"
        :dark="theme === 'dark'"
      />
    </div>
  </aside>
</template>
