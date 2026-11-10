<script lang="ts">
  import { onMount } from "svelte";
  import Icon from "../../../project/ui/atoms/icon/ui/Icon.svelte";
  import { bootswatchUi } from "../data/bootswatch-ui.svelte";
  import type { BootswatchThemeSelection } from "../../../project/data/style.component";

  let {
    compact = false,
    align = "left",
  }: {
    compact?: boolean;
    align?: "left" | "right";
  } = $props();

  let open = $state(false);
  let query = $state("");

  onMount(() => {
    bootswatchUi.init();
  });

  const filteredThemes = $derived(
    bootswatchUi.themes.filter((item) => {
      const q = query.trim().toLowerCase();
      if (!q) return true;
      return (
        item.name.toLowerCase().includes(q) ||
        item.description.toLowerCase().includes(q)
      );
    }),
  );

  function selectTheme(id: BootswatchThemeSelection) {
    bootswatchUi.setTheme(id);
    open = false;
  }
</script>

<div class="relative inline-block">
  <button
    type="button"
    class={compact
      ? "inline-flex h-7 items-center gap-1.5 rounded border border-blender-border bg-blender-surface px-2 text-[11px] font-medium text-zinc-200 transition-colors hover:bg-blender-hover"
      : "flex items-center gap-1.5 rounded px-2.5 py-1.5 text-zinc-300 transition-colors hover:bg-blender-hover hover:text-zinc-100"}
    aria-expanded={open}
    onclick={() => (open = !open)}
    title="Seleccionar tema Bootswatch"
  >
    {#if bootswatchUi.activePalette}
      <span
        class="size-2.5 shrink-0 rounded-full border border-white/30"
        style={`background-color: ${bootswatchUi.activePalette.primary}`}
      ></span>
      <span class="max-w-24 truncate">
        {compact ? bootswatchUi.activePalette.name : `Tema: ${bootswatchUi.activePalette.name}`}
      </span>
    {:else}
      <span class="size-2.5 shrink-0 rounded-full bg-blue-500"></span>
      <span>{compact ? "Tema" : "Temas (26)"}</span>
    {/if}
    <Icon
      name={open ? "ArrowUp" : "ArrowDown"}
      className="inline-flex h-3 w-3 shrink-0 items-center justify-center text-[10px] opacity-75"
    />
  </button>

  {#if open}
    <div
      class={`
        absolute top-full z-50 mt-1.5 w-80 rounded-lg border border-blender-border
        bg-blender-card p-2.5 shadow-2xl
        ${align === "right" ? "right-0" : "left-0"}
      `}
    >
      <div class="mb-2 space-y-1.5 border-b border-blender-border pb-2">
        <div class="flex items-center justify-between text-[11px] font-semibold text-zinc-300">
          <span>Aplicar tema em:</span>
          <span class="text-[10px] text-zinc-400">
            {bootswatchUi.scope === "app" ? "Toda a aplicação" : "Apenas src/project"}
          </span>
        </div>
        <div class="grid grid-cols-2 gap-1 rounded-md bg-blender-surface p-1 text-[11px]">
          <button
            type="button"
            class={`rounded px-2 py-1 font-medium transition-colors ${
              bootswatchUi.scope === "app"
                ? "bg-blue-600 text-white"
                : "text-zinc-400 hover:text-zinc-200"
            }`}
            onclick={() => bootswatchUi.setScope("app")}
          >
            Toda a aplicação
          </button>
          <button
            type="button"
            class={`rounded px-2 py-1 font-medium transition-colors ${
              bootswatchUi.scope === "project"
                ? "bg-blue-600 text-white"
                : "text-zinc-400 hover:text-zinc-200"
            }`}
            onclick={() => bootswatchUi.setScope("project")}
          >
            Só src/project
          </button>
        </div>
      </div>

      <input
        bind:value={query}
        placeholder="Filtrar 26 temas Bootswatch…"
        class="mb-2 w-full rounded border border-blender-border bg-blender-surface px-2.5 py-1.5 text-xs text-zinc-200 placeholder:text-zinc-500 focus:border-blue-500 focus:outline-none"
      />

      <div class="max-h-72 space-y-1 overflow-y-auto pr-0.5">
        <button
          type="button"
          class={`flex w-full items-center justify-between gap-2 rounded-md px-2.5 py-2 text-left text-xs transition-colors ${
            bootswatchUi.theme === "default"
              ? "bg-blue-600/20 font-semibold text-blue-400"
              : "text-zinc-200 hover:bg-blender-hover"
          }`}
          onclick={() => selectTheme("default")}
        >
          <div class="flex items-center gap-2">
            <span class="flex items-center gap-0.5">
              <span class="size-3 rounded-full bg-[#12121a] border border-white/20"></span>
              <span class="size-3 rounded-full bg-[#3b82f6]"></span>
            </span>
            <div>
              <div class="font-medium">Padrão (Blender)</div>
              <div class="text-[10px] text-zinc-400">Tema original NegoDesign</div>
            </div>
          </div>
          <span class="rounded bg-blender-surface px-1.5 py-0.5 text-[10px] text-zinc-400">
            Default
          </span>
        </button>

        {#each filteredThemes as item (item.id)}
          <button
            type="button"
            class={`flex w-full items-center justify-between gap-2 rounded-md px-2.5 py-2 text-left text-xs transition-colors ${
              bootswatchUi.theme === item.id
                ? "bg-blue-600/20 font-semibold text-blue-400"
                : "text-zinc-200 hover:bg-blender-hover"
            }`}
            onclick={() => selectTheme(item.id as BootswatchThemeSelection)}
          >
            <div class="flex min-w-0 items-center gap-2.5">
              <span class="flex shrink-0 items-center gap-0.5">
                <span
                  class="size-3 rounded-full border border-white/25"
                  style={`background-color: ${item.bodyBg}`}
                ></span>
                <span
                  class="size-3 rounded-full border border-white/25"
                  style={`background-color: ${item.primary}`}
                ></span>
                <span
                  class="size-3 rounded-full border border-white/25"
                  style={`background-color: ${item.secondary}`}
                ></span>
              </span>
              <div class="min-w-0">
                <div class="truncate font-medium">{item.name}</div>
                <div class="truncate text-[10px] text-zinc-400">{item.description}</div>
              </div>
            </div>
            <span class="shrink-0 rounded bg-blender-surface px-1.5 py-0.5 text-[10px] text-zinc-400">
              {item.isDark ? "Dark" : "Light"}
            </span>
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>

