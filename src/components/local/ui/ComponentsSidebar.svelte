<script lang="ts">
  import { onMount } from "svelte";
  import Icon from "../../../project/ui/atoms/icon/ui/Icon.svelte";
  import { IconNamed, getIconClass } from "../../../hooks/icon.svelte";
  import { StyleComponent } from "../data/style.component";
  import {
    componentNavigationGroups,
    type ComponentDefinition,
    type ComponentNavigationGroup,
  } from "../data/components";

  function cloneGroup(
    group: ComponentNavigationGroup,
  ): ComponentNavigationGroup {
    return {
      ...group,
      items: group.items ? [...group.items] : [],
      children: group.children?.map(cloneGroup),
    };
  }

  let groups = $state<ComponentNavigationGroup[]>(
    componentNavigationGroups.map(cloneGroup),
  );
  let search = $state("");
  let theme = $state<"light" | "dark">("light");

  onMount(() => {
    const savedTheme = localStorage.getItem("negodesign-theme");
    theme = savedTheme === "dark" ? "dark" : "light";
    document.documentElement.dataset.theme = theme;
  });

  function toggleTheme() {
    theme = theme === "light" ? "dark" : "light";
    document.documentElement.dataset.theme = theme;
    localStorage.setItem("negodesign-theme", theme);
  }

  function toggle(group: ComponentNavigationGroup) {
    group.open = !group.open;
  }

  function displayName(item: ComponentDefinition) {
    return item.label?.trim() || item.name;
  }

  function matches(item: ComponentDefinition) {
    const q = search.trim().toLowerCase();
    if (!q) return true;
    const ownMatch = (
      displayName(item).toLowerCase().includes(q) ||
      item.name.toLowerCase().includes(q)
    );
    return ownMatch || (item.children ?? []).some(matches);
  }

  function filtered(group: ComponentNavigationGroup) {
    return (group.items ?? []).filter(matches);
  }

  function hasResults(group: ComponentNavigationGroup): boolean {
    if (!search.trim()) return true;
    return (
      (group.items ?? []).some(matches) ||
      (group.children ?? []).some(hasResults)
    );
  }
</script>

{#snippet groupTree(group: ComponentNavigationGroup, depth: number)}
  {@const vis = filtered(group)}
  {#if hasResults(group)}
    <div class="border-b border-blender-border/40">
      <button
        type="button"
        onclick={() => toggle(group)}
        aria-expanded={group.open || Boolean(search.trim())}
        class="w-full flex items-center justify-between py-2 text-xs font-semibold
               uppercase tracking-widest text-zinc-500 hover:text-slate-900 hover:bg-blender-hover transition-colors"
        style="padding-left: {(0.75 + depth * 0.75).toFixed(
          2,
        )}rem; padding-right: 0.75rem"
      >
        <span>{group.label}</span>
        <Icon
          name={group.open || search.trim() ? "ArrowUp" : "ArrowDown"}
          className="inline-flex h-3 w-3 shrink-0 items-center justify-center self-center text-[10px] leading-none opacity-75"
        />
      </button>
      {#if group.open || search.trim()}
        {#if vis.length}
          <ul class="pb-1">
            {#each vis as item (item.id)}
              <li>
                {#if item.href}
                  <a
                    href={item.href}
                    class="{StyleComponent.layout
                      .item} w-full text-left gap-2 py-2"
                    style="padding-left: {(0.75 + depth * 0.75).toFixed(
                      2,
                    )}rem; padding-right: 0.75rem"
                    title={displayName(item)}
                  >
                    <i
                      class={`${getIconClass(item.icon)} h-5 w-5 text-base opacity-75 shrink-0`}
                      aria-hidden="true"
                    ></i>
                    <span class="text-sm truncate">{displayName(item)}</span>
                  </a>
                  {#if item.children?.length}
                    <ul class="border-l border-blender-border/70 pb-1 ml-5">
                      {#each item.children.filter(matches) as child (child.id)}
                        <li>
                          {#if child.href}
                            <a
                              href={child.href}
                              class="{StyleComponent.layout.item} w-full text-left gap-2 py-1.5 text-zinc-400 hover:text-zinc-100"
                              style="padding-left: {(0.75 + (depth + 1) * 0.75).toFixed(2)}rem; padding-right: 0.75rem"
                              title={displayName(child)}
                            >
                              <i
                                class={`${getIconClass(child.icon)} h-4 w-4 text-sm opacity-65 shrink-0`}
                                aria-hidden="true"
                              ></i>
                              <span class="text-xs truncate">{displayName(child)}</span>
                            </a>
                          {/if}
                        </li>
                      {/each}
                    </ul>
                  {/if}
                {:else}
                  <button
                    class="{StyleComponent.layout
                      .item} w-full text-left gap-2 py-2"
                    style="padding-left: {(0.75 + depth * 0.75).toFixed(
                      2,
                    )}rem; padding-right: 0.75rem"
                    draggable="true"
                    title={displayName(item)}
                  >
                    <i
                      class={`${getIconClass(item.icon)} h-5 w-5 text-base opacity-75 shrink-0`}
                      aria-hidden="true"
                    ></i>
                    <span class="text-sm truncate">{displayName(item)}</span>
                  </button>
                {/if}
              </li>
            {/each}
          </ul>
        {/if}
        {#if group.children?.length}
          {#each group.children as child (child.label)}
            {@render groupTree(child, depth + 1)}
          {/each}
        {/if}
      {/if}
    </div>
  {/if}
{/snippet}

<aside
  class="{StyleComponent.layout
    .sidebar} flex flex-col w-60 min-w-40 h-full overflow-hidden"
>
  <div
    class="{StyleComponent.layout.topbar} px-3 border-b border-blender-border"
  >
    <span class="text-xs font-semibold uppercase tracking-widest text-zinc-500"
      >Library</span
    >
    <div class="flex items-center gap-1">
      <button
        class="{StyleComponent.layout.themeToggle} {theme === 'light'
          ? StyleComponent.layout.themeToggleLight
          : StyleComponent.layout.themeToggleDark}"
        type="button"
        onclick={toggleTheme}
        title={theme === "light" ? "Ativar modo escuro" : "Ativar modo claro"}
        aria-label={theme === "light"
          ? "Ativar modo escuro"
          : "Ativar modo claro"}>{theme === "light" ? "☾" : "☀"}</button
      >
      <button
        class="{StyleComponent.layout.iconButton} h-7 w-7"
        title="Settings"
      >
        <i
          class={`${getIconClass(IconNamed.HelpSquare)} text-base opacity-75`}
          aria-hidden="true"
        ></i>
      </button>
    </div>
  </div>

  <div class="px-2 py-1.5 border-b border-blender-border">
    <input
      bind:value={search}
      placeholder="Search…"
      class="w-full bg-blender-base border border-blender-border rounded px-2.5 py-2 text-sm
             text-zinc-300 placeholder:text-zinc-500 focus:outline-none focus:border-blue-500/50 transition-colors"
    />
  </div>

  <div class="flex-1 overflow-y-auto">
    {#each groups as group (group.label)}
      {@render groupTree(group, 0)}
    {/each}
  </div>
</aside>
