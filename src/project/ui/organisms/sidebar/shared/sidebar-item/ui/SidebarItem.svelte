<script lang="ts">
  import Icon from "../../../../../atoms/icon/ui/Icon.svelte";
  import type { SidebarItemProps } from "../../../data/types";

  let {
    item,
    compact = false,
    dark = false,
    accent = "blue",
  }: {
    item: SidebarItemProps;
    compact?: boolean;
    dark?: boolean;
    accent?: string;
  } = $props();

  // svelte-ignore state_referenced_locally -- Initial expanded state follows the active navigation item.
  let expanded = $state(item.active ?? false);

  function handleClick(event: MouseEvent) {
    if (!item.children?.length) return;
    event.preventDefault();
    expanded = !expanded;
  }

  const activeLightClass = $derived(
    accent === "violet"
      ? "bg-violet-50 text-violet-700"
      : "bg-blue-50 text-blue-700",
  );
</script>

<div class="space-y-1">
  <a
    href={item.href ?? "#"}
    title={compact ? item.label : undefined}
    aria-current={item.active ? "page" : undefined}
    onclick={handleClick}
    class={`group flex min-h-9 items-center gap-3 rounded-lg px-3 text-sm transition ${compact ? "justify-center px-2" : ""} ${item.active ? (dark ? "bg-zinc-700 text-white" : activeLightClass) : dark ? "text-zinc-300 hover:bg-zinc-800 hover:text-white" : "text-zinc-600 hover:bg-zinc-100 hover:text-zinc-950"}`}
  >
    {#if item.icon}
      <Icon name={item.icon} className="shrink-0 text-base" />
    {/if}

    {#if !compact}
      <span class="min-w-0 flex-1 truncate">{item.label}</span>
      {#if item.badge}
        <span
          class={`rounded-md px-1.5 py-0.5 text-[10px] ${dark ? "bg-zinc-800 text-zinc-300" : "bg-zinc-100 text-zinc-600"}`}
        >
          {item.badge}
        </span>
      {/if}
      {#if item.children?.length}
        <Icon name={expanded ? "ArrowUp" : "ArrowDown"} className="text-xs" />
      {/if}
    {/if}
  </a>

  {#if item.children?.length && expanded && !compact}
    <div class="ml-7 space-y-1 border-l border-current/10 pl-3">
      {#each item.children as child (child.label)}
        <a
          href={child.href ?? "#"}
          class={`flex min-h-8 items-center justify-between rounded-md px-2 text-xs ${child.active ? (dark ? "bg-zinc-800 text-white" : activeLightClass) : dark ? "text-zinc-400 hover:text-white" : "text-zinc-500 hover:text-zinc-900"}`}
        >
          <span>{child.label}</span>
          {#if child.badge}<span>{child.badge}</span>{/if}
        </a>
      {/each}
    </div>
  {/if}
</div>

