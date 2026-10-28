<script lang="ts">
  import Icon from "../../../project/ui/atoms/icon/ui/Icon.svelte";
  import {
    componentNavigationGroups,
    type ComponentNavigationGroup,
  } from "../data/components";

  let componentsOpen = $state(false);
  let openGroups: Record<string, boolean> = $state({});

  function isGroupOpen(label: string) {
    return openGroups[label] ?? label === "Input";
  }

  function hasDocumented(group: ComponentNavigationGroup): boolean {
    if ((group.items ?? []).some((item) => item.href)) return true;
    return (group.children ?? []).some(hasDocumented);
  }
</script>

{#snippet navGroup(group: ComponentNavigationGroup)}
  {#if hasDocumented(group)}
    <details
      class="group"
      open={isGroupOpen(group.label)}
      ontoggle={(event) => (openGroups[group.label] = event.currentTarget.open)}
    >
      <summary
        class="flex cursor-pointer list-none items-center justify-between rounded px-2 py-1.5 text-[10px] font-semibold uppercase tracking-wider text-zinc-500 transition-colors hover:bg-blender-hover hover:text-zinc-200 [&::-webkit-details-marker]:hidden"
      >
        <span>{group.label}</span>
        <Icon
          name={isGroupOpen(group.label) ? "ArrowUp" : "ArrowDown"}
          className="h-3 w-3 text-[10px] opacity-70"
        />
      </summary>
      <ul class="pb-1">
        {#each (group.items ?? []).filter((item) => item.href) as item (item.id)}
          <li>
            <a
              href={item.href}
              class="flex items-center gap-2 rounded px-2 py-1.5 text-xs text-zinc-300 transition-colors hover:bg-blender-hover hover:text-zinc-100"
              title={item.label ?? item.name}
              onclick={() => (componentsOpen = false)}
            >
              <span class="truncate">{item.label ?? item.name}</span>
              <span class="ml-auto text-[10px] text-zinc-500">{item.group}</span
              >
            </a>
          </li>
        {/each}
      </ul>
      {#if group.children?.length}
        <div class="pl-2">
          {#each group.children as child (child.label)}
            {@render navGroup(child)}
          {/each}
        </div>
      {/if}
    </details>
  {/if}
{/snippet}

<nav
  class="flex min-h-7 w-full flex-none items-center gap-1 border-b border-blender-border px-2 font-commissioner text-xs"
  aria-label="Menu principal"
>
  <a
    href="/"
    class="rounded px-2.5 py-1.5 text-zinc-300 transition-colors hover:bg-blender-hover hover:text-zinc-100"
  >
    Início
  </a>
  <a
    href="/main/project"
    class="rounded px-2.5 py-1.5 text-zinc-300 transition-colors hover:bg-blender-hover hover:text-zinc-100"
  >
    Projecto
  </a>

  <div class="relative">
    <button
      type="button"
      class="flex items-center gap-1 rounded px-2.5 py-1.5 text-zinc-300 transition-colors hover:bg-blender-hover hover:text-zinc-100"
      aria-expanded={componentsOpen}
      onclick={() => (componentsOpen = !componentsOpen)}
    >
      Componentes
      <Icon
        name={componentsOpen ? "ArrowUp" : "ArrowDown"}
        className="inline-flex h-3.5 w-3.5 shrink-0 items-center justify-center self-center text-[11px] leading-none opacity-75 transition-colors"
      />
    </button>

    {#if componentsOpen}
      <div
        class="absolute left-0 top-full z-50 max-h-[70vh] w-64 overflow-y-auto rounded-md border border-blender-border bg-blender-card p-1.5 shadow-xl"
      >
        {#each componentNavigationGroups as group (group.label)}
          {@render navGroup(group)}
        {/each}
      </div>
    {/if}
  </div>
</nav>
