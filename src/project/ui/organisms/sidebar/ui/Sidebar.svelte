<script lang="ts">
  import Icon from "../../../atoms/icon/ui/Icon.svelte";
  import SidebarProfile from "../shared/sidebar-profile/ui/SidebarProfile.svelte";
  import SidebarSection from "../shared/sidebar-section/ui/SidebarSection.svelte";
  import {
    computeSidebarTheme,
    SIDEBAR_RAIL_FOOTER_ICONS,
    SIDEBAR_RAIL_ICONS,
  } from "../../../../hooks/sidebar.hook";
  import type { SidebarProps } from "../data/types";

  let {
    variant = "01",
    theme = "light",
    collapsed = false,
    logo = "◈",
    brand = "Untitled UI",
    workspace = "workspace.untitledui.com",
    sections = [],
    footerSections = [],
    user = { name: "Sienna Hewitt", detail: "sienna@untitledui.com" },
    storage,
    className = "",
  }: SidebarProps = $props();

  const cfg = $derived(computeSidebarTheme(variant, theme, collapsed, className));
  let query = $state("");
</script>

<aside class={cfg.rootClass} aria-label={`${brand} sidebar`}>
  {#if cfg.rail}
    <nav class={cfg.railClass} aria-label="Shortcuts">
      <div class="grid h-9 w-9 place-items-center rounded-xl bg-blue-600 text-lg font-bold text-white">∞</div>
      <div class="my-2 h-8 w-8 rounded-full bg-blue-100 ring-2 ring-blue-500"></div>
      {#each SIDEBAR_RAIL_ICONS as icon, i}
        <button class={`grid h-9 w-9 place-items-center rounded-lg ${i === 3 ? "bg-blue-600 text-white" : cfg.dark ? "text-zinc-400 hover:bg-zinc-800" : "text-zinc-500 hover:bg-zinc-200"}`} aria-label={icon}>
          <Icon name={icon} />
        </button>
      {/each}
      <div class="flex-1"></div>
      {#each SIDEBAR_RAIL_FOOTER_ICONS as icon}
        <button class="grid h-8 w-8 place-items-center rounded-lg text-zinc-500" aria-label={icon}><Icon name={icon} /></button>
      {/each}
    </nav>
  {/if}

  <div class="flex min-w-0 flex-1 flex-col p-4">
    <header class="mb-4 flex min-h-9 items-center gap-2">
      {#if !cfg.rail}<span class={cfg.logoBoxClass}>{logo}</span>{/if}
      {#if !cfg.compact}
        <div class="min-w-0 flex-1">
          <p class="truncate text-sm font-bold">{brand}</p>
          {#if variant === "04"}<p class="truncate text-[10px] text-zinc-500">{workspace}</p>{/if}
        </div>
        {#if variant === "04"}<button aria-label="Switch workspace" class="text-zinc-500">⌄</button>{/if}
      {/if}
    </header>

    {#if cfg.showSearch}
      <label class={cfg.searchClass}>
        <Icon name="Search" className="text-zinc-400" />
        <input bind:value={query} placeholder={cfg.compact ? "" : "Search"} aria-label="Search"
          class="w-full min-w-0 bg-transparent text-xs outline-none placeholder:text-zinc-400" />
        {#if !cfg.compact}<kbd class="rounded border px-1 text-[9px] text-zinc-400">⌘ K</kbd>{/if}
      </label>
    {/if}

    <div class="min-h-0 flex-1 space-y-3 overflow-y-auto">
      {#each sections as section, idx (`${section.title ?? "sec"}-${idx}`)}
        <SidebarSection {section} compact={cfg.compact} dark={cfg.dark} accent={cfg.accent} />
      {/each}
    </div>

    {#if storage && !cfg.compact}
      <div class={cfg.storageClass}>
        <p class="text-xs font-semibold">{storage.title}</p>
        <p class="mt-1 text-[10px] leading-4 text-zinc-500">{storage.description}</p>
        <div class="mt-3 h-1.5 overflow-hidden rounded-full bg-zinc-200 dark:bg-zinc-700">
          <div class="h-full rounded-full bg-blue-600" style={`width: ${Math.min(100, Math.max(0, storage.usage))}%`}></div>
        </div>
      </div>
    {/if}

    {#if footerSections.length}
      <div class="space-y-1 border-t border-current/10 py-3">
        {#each footerSections as section, idx (`f-${idx}`)}
          <SidebarSection {section} compact={cfg.compact} dark={cfg.dark} />
        {/each}
      </div>
    {/if}
    <SidebarProfile {user} compact={cfg.compact} dark={cfg.dark} />
  </div>
</aside>
