<script lang="ts">
  import { copyComponentToActiveProject } from "../../../data/component-copy";
  import type { ExportThemeOption } from "../../../data/copy/core";
  import { bootswatchUi } from "../../../data/bootswatch-ui.svelte";
  import {
    BOOTSWATCH_THEME_LIST,
    getBootswatchTheme,
  } from "../../../../../project/data/style.component";
  import { locales } from "../../../../../project/data/i18n";
  import type { ComponentDefinition } from "../../../data/components";

  let {
    component,
    onclose,
    onmessage,
  }: {
    component: ComponentDefinition;
    onclose: () => void;
    onmessage: (message: string) => void;
  } = $props();

  let selectedLocales = $state<string[]>([...locales]);
  let selectedTheme = $state<ExportThemeOption>(bootswatchUi.theme ?? "default");
  let themeSearch = $state("");
  let copying = $state(false);
  let errorMessage = $state("");
  const localeNames: Record<string, string> = { en: "English", pt: "Português" };

  const filteredThemes = $derived(
    BOOTSWATCH_THEME_LIST.filter((t) => {
      const q = themeSearch.trim().toLowerCase();
      if (!q) return true;
      return (
        t.name.toLowerCase().includes(q) ||
        t.description.toLowerCase().includes(q)
      );
    }),
  );

  function themeLabel(theme: ExportThemeOption): string {
    if (theme === "default") return "Padrão (Blender)";
    if (theme === "all") return "Todos os 26 temas Bootswatch";
    return getBootswatchTheme(theme)?.name ?? theme;
  }

  async function copy() {
    if (!component.dependence || !selectedLocales.length) return;
    copying = true;
    errorMessage = "";
    let completed = false;
    try {
      const result = await copyComponentToActiveProject(component.id, {
        locales: selectedLocales,
        theme: selectedTheme,
      });
      const languages = selectedLocales
        .map((language) => localeNames[language] ?? language)
        .join(", ");
      const exportedThemeName = themeLabel(selectedTheme);
      onmessage(
        `${component.label ?? component.name}: ${result.copiedFiles} ficheiro(s), ${result.exportedComponents} componente(s), tema "${exportedThemeName}" e idiomas (${languages}) exportados para ${result.project.name}/negodesign.`,
      );
      completed = true;
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
    } finally {
      copying = false;
      if (completed) onclose();
    }
  }
</script>

<div
  class="fixed inset-0 z-[100] flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm"
  role="presentation"
  onclick={(event) => event.target === event.currentTarget && !copying && onclose()}
  onkeydown={(event) => event.key === "Escape" && !copying && onclose()}
>
  <div
    class="flex max-h-[90vh] w-full max-w-lg flex-col overflow-hidden rounded-xl border border-blender-border bg-blender-card p-5 shadow-2xl"
    role="dialog"
    aria-modal="true"
    aria-labelledby="copy-title"
    aria-describedby="copy-description"
    tabindex="-1"
  >
    <div class="flex items-start justify-between gap-4">
      <div>
        <h2 id="copy-title" class="text-base font-semibold text-zinc-100">
          Exportar {component.label ?? component.name} para o projecto
        </h2>
        <p id="copy-description" class="mt-1 text-xs leading-5 text-zinc-400">
          Escolha o tema Bootswatch e os idiomas que devem ser exportados com o componente.
        </p>
      </div>
      <button
        type="button"
        class="rounded px-2 py-1 text-zinc-400 hover:bg-blender-hover hover:text-zinc-100"
        onclick={onclose}
        aria-label="Fechar"
        disabled={copying}
      >
        ×
      </button>
    </div>

    <div class="mt-4 min-h-0 flex-1 space-y-5 overflow-y-auto pr-1">
      <fieldset class="space-y-2.5" disabled={copying}>
        <legend class="flex w-full items-center justify-between gap-3 text-xs font-semibold text-zinc-200">
          <span>Tema a exportar</span>
          <span class="rounded bg-blue-500/15 px-2 py-0.5 text-[11px] font-medium text-blue-400">
            {themeLabel(selectedTheme)}
          </span>
        </legend>

        <div class="grid grid-cols-1 gap-2 sm:grid-cols-2">
          <button
            type="button"
            onclick={() => (selectedTheme = "default")}
            class={`flex items-center justify-between gap-2 rounded-lg border px-3 py-2.5 text-left text-xs transition-colors ${
              selectedTheme === "default"
                ? "border-blue-500 bg-blue-500/15 text-zinc-100"
                : "border-blender-border bg-blender-base text-zinc-300 hover:border-zinc-500"
            }`}
          >
            <div class="flex items-center gap-2">
              <span class="flex items-center gap-0.5">
                <span class="size-3 rounded-full border border-white/20 bg-[#12121a]"></span>
                <span class="size-3 rounded-full bg-[#3b82f6]"></span>
              </span>
              <div>
                <div class="font-medium">Padrão (Blender)</div>
                <div class="text-[10px] text-zinc-400">Tema original NegoDesign</div>
              </div>
            </div>
          </button>

          <button
            type="button"
            onclick={() => (selectedTheme = "all")}
            class={`flex items-center justify-between gap-2 rounded-lg border px-3 py-2.5 text-left text-xs transition-colors ${
              selectedTheme === "all"
                ? "border-blue-500 bg-blue-500/15 text-zinc-100"
                : "border-blender-border bg-blender-base text-zinc-300 hover:border-zinc-500"
            }`}
          >
            <div>
              <div class="font-medium">Todos os Temas (26)</div>
              <div class="text-[10px] text-zinc-400">Exportar pacote completo Bootswatch</div>
            </div>
            <span class="rounded bg-blender-surface px-1.5 py-0.5 text-[10px] text-zinc-400">
              All
            </span>
          </button>
        </div>

        <input
          bind:value={themeSearch}
          placeholder="Pesquisar um tema Bootswatch específico (ex.: Cerulean, Darkly, Vapor)…"
          class="w-full rounded-md border border-blender-border bg-blender-base px-3 py-1.5 text-xs text-zinc-200 placeholder:text-zinc-500 focus:border-blue-500 focus:outline-none"
        />

        <div class="grid max-h-48 grid-cols-1 gap-1.5 overflow-y-auto pr-1 sm:grid-cols-2">
          {#each filteredThemes as item (item.id)}
            {@const isChosen = selectedTheme === item.id}
            <button
              type="button"
              onclick={() => (selectedTheme = item.id as ExportThemeOption)}
              class={`flex items-center justify-between gap-2 rounded-lg border px-2.5 py-2 text-left text-xs transition-colors ${
                isChosen
                  ? "border-blue-500 bg-blue-500/15 text-zinc-100"
                  : "border-blender-border bg-blender-base text-zinc-300 hover:border-zinc-500"
              }`}
            >
              <div class="flex min-w-0 items-center gap-2">
                <span class="flex shrink-0 items-center gap-0.5">
                  <span
                    class="size-2.5 rounded-full border border-white/25"
                    style={`background-color: ${item.bodyBg}`}
                  ></span>
                  <span
                    class="size-2.5 rounded-full border border-white/25"
                    style={`background-color: ${item.primary}`}
                  ></span>
                  <span
                    class="size-2.5 rounded-full border border-white/25"
                    style={`background-color: ${item.secondary}`}
                  ></span>
                </span>
                <div class="min-w-0">
                  <div class="truncate font-medium">{item.name}</div>
                  <div class="truncate text-[10px] text-zinc-400">{item.description}</div>
                </div>
              </div>
              <span class="shrink-0 rounded bg-blender-surface px-1.5 py-0.5 text-[9px] text-zinc-400">
                {item.isDark ? "Dark" : "Light"}
              </span>
            </button>
          {/each}
        </div>
      </fieldset>

      <fieldset class="space-y-2 border-t border-blender-border pt-4" disabled={copying}>
        <legend class="flex w-full justify-between gap-3 text-xs font-semibold text-zinc-200">
          <span>Idiomas a incluir</span>
          <span class="text-[10px] font-normal text-zinc-500">
            {selectedLocales.length} seleccionado(s)
          </span>
        </legend>
        <div class="grid gap-2 sm:grid-cols-2">
          {#each locales as language (language)}
            {@const selected = selectedLocales.includes(language)}
            <label
              class="flex cursor-pointer items-center gap-3 rounded-lg border px-3 py-2.5 {selected
                ? 'border-blue-500/60 bg-blue-500/10'
                : 'border-blender-border bg-blender-base hover:border-zinc-500'}"
            >
              <input
                type="checkbox"
                value={language}
                bind:group={selectedLocales}
                class="h-4 w-4 accent-blue-500"
              />
              <span class="flex-1 text-xs text-zinc-200">
                {localeNames[language] ?? language}
                <span class="block text-[10px] uppercase text-zinc-500">{language}</span>
              </span>
              {#if selected}<span class="text-[11px] text-blue-400">Incluído</span>{/if}
            </label>
          {/each}
        </div>
      </fieldset>
    </div>

    {#if errorMessage}
      <p class="mt-3 text-xs text-rose-400" role="alert">{errorMessage}</p>
    {/if}

    <div class="mt-5 flex justify-end gap-2 border-t border-blender-border pt-3">
      <button
        type="button"
        class="rounded-md border border-blender-border px-3 py-2 text-xs text-zinc-300 hover:bg-blender-hover"
        onclick={onclose}
        disabled={copying}
      >
        Cancelar
      </button>
      <button
        type="button"
        class="rounded-md bg-blue-600 px-3.5 py-2 text-xs font-semibold text-white hover:bg-blue-500 disabled:opacity-50"
        onclick={copy}
        disabled={!selectedLocales.length || copying}
      >
        {copying ? "A exportar…" : "Exportar para projecto"}
      </button>
    </div>
  </div>
</div>
