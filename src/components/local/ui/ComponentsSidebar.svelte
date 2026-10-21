<script lang="ts">
  import { onMount } from "svelte";
  import { IconNamed, getIconClass } from "../../../hooks/icon.svelte";
  import {
    componentNavigationGroups,
    type ComponentDefinition,
    type ComponentNavigationGroup,
  } from "../data/components";

  let groups = $state<ComponentNavigationGroup[]>(
    componentNavigationGroups.map((group) => ({
      ...group,
      items: [...group.items],
    })),
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

  function filtered(items: ComponentNavigationGroup["items"]) {
    if (!search.trim()) return items;
    return items.filter((item) =>
      displayName(item).toLowerCase().includes(search.toLowerCase()) || item.name.toLowerCase().includes(search.toLowerCase()),
    );
  }
</script>

<aside
  class="blender-sidebar flex flex-col w-50 min-w-40 h-full overflow-hidden"
>
  <div class="blender-topbar px-3 border-b border-blender-border">
    <span class="text-xs font-semibold uppercase tracking-widest text-zinc-500"
      >Library</span
    >
    <div class="flex items-center gap-1">
      <button
        class="blender-icon-btn h-7 w-7 text-base"
        type="button"
        onclick={toggleTheme}
        title={theme === "light" ? "Ativar modo escuro" : "Ativar modo claro"}
        aria-label={theme === "light"
          ? "Ativar modo escuro"
          : "Ativar modo claro"}>{theme === "light" ? "☾" : "☀"}</button
      >
      <button class="blender-icon-btn h-7 w-7" title="Settings">
        <i class={`${getIconClass(IconNamed.Settings01)} text-base opacity-75`} aria-hidden="true"></i>
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
    {#each groups as group}
      {@const vis = filtered(group.items)}
      {#if !search.trim() || vis.length}
        <div class="border-b border-blender-border/40">
          <button
            onclick={() => toggle(group)}
            class="w-full flex items-center justify-between px-3 py-2 text-xs font-semibold
                   uppercase tracking-widest text-zinc-500 hover:text-slate-900 hover:bg-blender-hover transition-colors"
          >
            <span>{group.label}</span>
            <span
              class="text-[10px]"
              style="transform: rotate({group.open
                ? 0
                : -90}deg); display:inline-block; transition: transform .2s"
              >▼</span
            >
          </button>
          {#if group.open || search.trim()}
            <ul class="pb-1">
              {#each vis as item}
                <li>
                  {#if item.href}
                    <a
                      href={item.href}
                      class="blender-item w-full text-left gap-2 px-3 py-2"
                      title={displayName(item)}
                    >
                      <i class={`${getIconClass(item.icon)} h-5 w-5 text-base opacity-75 shrink-0`} aria-hidden="true"></i>
                      <span class="text-sm truncate">{displayName(item)}</span>
                    </a>
                  {:else}
                    <button
                      class="blender-item w-full text-left gap-2 px-3 py-2"
                      draggable="true"
                      title={displayName(item)}
                    >
                      <i class={`${getIconClass(item.icon)} h-5 w-5 text-base opacity-75 shrink-0`} aria-hidden="true"></i>
                      <span class="text-sm truncate">{displayName(item)}</span>
                    </button>
                  {/if}
                </li>
              {/each}
            </ul>
          {/if}
        </div>
      {/if}
    {/each}
  </div>
</aside>
