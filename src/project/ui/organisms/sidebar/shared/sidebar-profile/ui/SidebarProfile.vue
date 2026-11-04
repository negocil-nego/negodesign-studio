<script setup lang="ts">
import type { SidebarProfileProps } from "../../../data/types";
withDefaults(
  defineProps<{
    user: SidebarProfileProps;
    compact?: boolean;
    dark?: boolean;
  }>(),
  { compact: false, dark: false },
);
</script>
<template>
  <div
    :title="compact ? user.name : undefined"
    :class="`flex min-w-0 items-center gap-3 rounded-xl border p-2 ${compact ? 'justify-center' : ''} ${dark ? 'border-zinc-700 bg-zinc-800/70' : 'border-zinc-200 bg-white'}`"
  >
    <img
      v-if="user.avatar"
      :src="user.avatar"
      alt=""
      class="h-9 w-9 shrink-0 rounded-lg object-cover"
    /><span
      v-else
      class="grid h-9 w-9 shrink-0 place-items-center rounded-lg bg-blue-100 text-xs font-semibold text-blue-700"
      >{{
        user.name
          .split(" ")
          .map((x) => x[0])
          .slice(0, 2)
          .join("")
          .toUpperCase()
      }}</span
    >
    <div v-if="!compact" class="min-w-0 flex-1">
      <p
        :class="`truncate text-xs font-semibold ${dark ? 'text-zinc-100' : 'text-zinc-800'}`"
      >
        {{ user.name }}
      </p>
      <p v-if="user.detail" class="truncate text-[10px] text-zinc-500">
        {{ user.detail }}
      </p>
    </div>
    <button v-if="!compact" aria-label="Opções do perfil">···</button>
  </div>
</template>
