<script lang="ts">
  import Icon from "../../../atoms/icon/ui/Icon.svelte";
  import SidebarProfile from "../shared/sidebar-profile/ui/SidebarProfile.svelte";
  import SidebarSection from "../shared/sidebar-section/ui/SidebarSection.svelte";
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

  const dark = $derived(theme === "dark");
  const compact = $derived(collapsed);
  const rail = $derived(variant === "02");
  const surface = $derived(
    dark
      ? "border-zinc-800 bg-zinc-950 text-zinc-100"
      : "border-zinc-200 bg-white text-zinc-900",
  );
  let query = $state("");
</script>

<aside
  class={`relative flex h-[min(760px,90vh)] min-h-[560px] shrink-0 flex-col overflow-hidden rounded-2xl border shadow-sm transition-all ${surface} ${rail ? "w-[min(100%,420px)] flex-row" : compact ? "w-[76px]" : "w-[min(100%,280px)]"} ${className}`}
  aria-label={`${brand} sidebar`}
>
  {#if rail}
    <nav
      class={`flex w-[68px] shrink-0 flex-col items-center gap-3 border-r p-3 ${dark ? "border-zinc-800 bg-zinc-900" : "border-zinc-100 bg-zinc-50"}`}
      aria-label="Atalhos"
    >
      <div
        class="grid h-9 w-9 place-items-center rounded-xl bg-blue-600 text-lg font-bold text-white"
      >
        ∞
      </div>
      <div class="my-2 h-8 w-8 rounded-full bg-blue-100 ring-2 ring-blue-500"></div>
      {#each ["Grid", "Search", "Star", "Home", "Mail", "User", "Settings"] as icon, i}
        <button
          class={`grid h-9 w-9 place-items-center rounded-lg ${i === 3 ? "bg-blue-600 text-white" : dark ? "text-zinc-400 hover:bg-zinc-800" : "text-zinc-500 hover:bg-zinc-200"}`}
          aria-label={icon}
        >
          <Icon name={icon} />
        </button>
      {/each}
      <div class="flex-1"></div>
      {#each ["Bell", "Settings"] as icon}
        <button
          class="grid h-8 w-8 place-items-center rounded-lg text-zinc-500"
          aria-label={icon}
        >
          <Icon name={icon} />
        </button>
      {/each}
    </nav>
  {/if}

  <div class="flex min-w-0 flex-1 flex-col p-4">
    <header class="mb-4 flex min-h-9 items-center gap-2">
      {#if !rail}
        <span
          class={`grid h-8 w-8 shrink-0 place-items-center rounded-lg text-lg font-bold ${dark ? "bg-zinc-800" : "bg-zinc-100"}`}
        >
          {logo}
        </span>
      {/if}
      {#if !compact}
        <div class="min-w-0 flex-1">
          <p class="truncate text-sm font-bold">{brand}</p>
          {#if variant === "04"}
            <p class={`truncate text-[10px] ${dark ? "text-zinc-400" : "text-zinc-500"}`}>
              {workspace}
            </p>
          {/if}
        </div>
        {#if variant === "04"}
          <button aria-label="Mudar espaço de trabalho" class="text-zinc-500">⌄</button>
        {/if}
      {/if}
    </header>

    {#if variant === "01" || variant === "02"}
      <label
        class={`mb-4 flex h-9 items-center gap-2 rounded-lg border px-2.5 ${dark ? "border-zinc-700 bg-zinc-900" : "border-zinc-200 bg-white"}`}
      >
        <Icon name="Search" className="text-zinc-400" />
        <input
          bind:value={query}
          placeholder={compact ? "" : "Pesquisar"}
          aria-label="Pesquisar"
          class="w-full min-w-0 bg-transparent text-xs outline-none placeholder:text-zinc-400"
        />
        {#if !compact}
          <kbd class="rounded border px-1 text-[9px] text-zinc-400">⌘ K</kbd>
        {/if}
      </label>
    {/if}

    <div class="min-h-0 flex-1 space-y-3 overflow-y-auto">
      {#each sections as section (section.title)}
        <SidebarSection
          {section}
          compact={compact}
          {dark}
          accent={variant === "03" ? "violet" : "blue"}
        />
      {/each}
    </div>

    {#if storage && !compact}
      <div
        class={`my-3 rounded-xl border p-3 ${dark ? "border-zinc-800 bg-zinc-900" : "border-zinc-200 bg-zinc-50"}`}
      >
        <p class="text-xs font-semibold">{storage.title}</p>
        <p class={`mt-1 text-[10px] leading-4 ${dark ? "text-zinc-400" : "text-zinc-500"}`}>
          {storage.description}
        </p>
        <div
          class={`mt-3 h-1.5 overflow-hidden rounded-full ${dark ? "bg-zinc-700" : "bg-zinc-200"}`}
        >
          <div
            class="h-full rounded-full bg-blue-600"
            style={`width: ${Math.min(100, Math.max(0, storage.usage))}%`}
          ></div>
        </div>
        <div class="mt-2 flex justify-between text-[10px]">
          <button class="text-zinc-500">Dispensar</button>
          <button class="font-semibold text-blue-600">Actualizar</button>
        </div>
      </div>
    {/if}

    {#if footerSections.length}
      <div class="space-y-1 border-t border-current/10 py-3">
        {#each footerSections as section (section.title)}
          <SidebarSection {section} compact={compact} {dark} />
        {/each}
      </div>
    {/if}

    <SidebarProfile {user} compact={compact} {dark} />
  </div>
</aside>
