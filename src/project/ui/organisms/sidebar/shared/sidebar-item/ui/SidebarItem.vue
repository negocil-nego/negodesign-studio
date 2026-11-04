<script setup lang="ts">
import { ref } from "vue";
import Icon from "../../../../../atoms/icon/ui/Icon.vue";
import type { SidebarItemProps } from "../../../data/types";
const props = withDefaults(
  defineProps<{
    item: SidebarItemProps;
    compact?: boolean;
    dark?: boolean;
    accent?: string;
  }>(),
  { compact: false, dark: false, accent: "blue" },
);
const open = ref(Boolean(props.item.active));
</script>
<template>
  <div class="space-y-1">
    <a
      :href="item.href ?? '#'"
      :title="compact ? item.label : undefined"
      :aria-current="item.active ? 'page' : undefined"
      @click="
        item.children?.length && ((open = !open), $event.preventDefault())
      "
      :class="`flex min-h-9 items-center gap-3 rounded-lg px-3 text-sm transition ${compact ? 'justify-center px-2' : ''} ${item.active ? (dark ? 'bg-zinc-700 text-white' : accent === 'violet' ? 'bg-violet-50 text-violet-700' : 'bg-blue-50 text-blue-700') : dark ? 'text-zinc-300 hover:bg-zinc-800 hover:text-white' : 'text-zinc-600 hover:bg-zinc-100 hover:text-zinc-950'}`"
      ><Icon v-if="item.icon" :name="item.icon" /><span
        v-if="!compact"
        class="min-w-0 flex-1 truncate"
        >{{ item.label }}</span
      ><small v-if="!compact && item.badge">{{ item.badge }}</small
      ><span v-if="!compact && item.children?.length">{{
        open ? "⌃" : "⌄"
      }}</span></a
    >
    <div
      v-if="open && !compact && item.children?.length"
      class="ml-7 space-y-1 border-l border-current/10 pl-3"
    >
      <a
        v-for="child in item.children"
        :key="child.label"
        :href="child.href ?? '#'"
        class="flex min-h-8 justify-between rounded-md px-2 text-xs text-zinc-500"
        >{{ child.label }}<span v-if="child.badge">{{ child.badge }}</span></a
      >
    </div>
  </div>
</template>
