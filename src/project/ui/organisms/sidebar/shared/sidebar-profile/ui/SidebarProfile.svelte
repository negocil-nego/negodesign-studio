<script lang="ts">
  import type { SidebarProfileProps } from "../../../data/types";

  let {
    user,
    compact = false,
    dark = false,
  }: {
    user: SidebarProfileProps;
    compact?: boolean;
    dark?: boolean;
  } = $props();

  const initials = $derived(
    user.name
      .split(" ")
      .map((part) => part[0])
      .slice(0, 2)
      .join("")
      .toUpperCase(),
  );
</script>

<div
  class={`flex min-w-0 items-center gap-3 rounded-xl border p-2 ${compact ? "justify-center" : ""} ${dark ? "border-zinc-700 bg-zinc-800/70" : "border-zinc-200 bg-white"}`}
  title={compact ? user.name : undefined}
>
  {#if user.avatar}
    <img src={user.avatar} alt="" class="h-9 w-9 shrink-0 rounded-lg object-cover" />
  {:else}
    <span
      class={`grid h-9 w-9 shrink-0 place-items-center rounded-lg text-xs font-semibold ${dark ? "bg-zinc-700 text-white" : "bg-blue-100 text-blue-700"}`}
    >
      {initials}
    </span>
  {/if}

  {#if !compact}
    <div class="min-w-0 flex-1">
      <p class={`truncate text-xs font-semibold ${dark ? "text-zinc-100" : "text-zinc-800"}`}>
        {user.name}
      </p>
      {#if user.detail}
        <p class={`truncate text-[10px] ${dark ? "text-zinc-400" : "text-zinc-500"}`}>
          {user.detail}
        </p>
      {/if}
    </div>
    <button type="button" aria-label="Opções do perfil" class="text-zinc-400">···</button>
  {/if}
</div>
