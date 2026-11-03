<script lang="ts">
  import { copyComponentToActiveProject } from "../../../data/component-copy";
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
  let copying = $state(false);
  let errorMessage = $state("");
  const localeNames: Record<string, string> = { en: "English", pt: "Português" };

  async function copy() {
    if (!component.dependence || !selectedLocales.length) return;
    copying = true;
    errorMessage = "";
    let completed = false;
    try {
      const result = await copyComponentToActiveProject(component.id, { locales: selectedLocales });
      const languages = selectedLocales.map((language) => localeNames[language] ?? language).join(", ");
      onmessage(`${component.label ?? component.name}: ${result.copiedFiles} ficheiro(s), ${result.exportedComponents} componente(s) e idiomas ${languages} disponíveis em ${result.project.name}/negodesign.`);
      completed = true;
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
    } finally {
      copying = false;
      if (completed) onclose();
    }
  }
</script>

<div class="fixed inset-0 z-[100] flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm" role="presentation" onclick={(event) => event.target === event.currentTarget && !copying && onclose()} onkeydown={(event) => event.key === "Escape" && !copying && onclose()}>
  <div class="w-full max-w-md rounded-xl border border-blender-border bg-blender-card p-5 shadow-2xl" role="dialog" aria-modal="true" aria-labelledby="copy-title" aria-describedby="copy-description" tabindex="-1">
    <div class="flex items-start justify-between gap-4">
      <div>
        <h2 id="copy-title" class="text-base font-semibold text-zinc-100">Internacionalizar componente</h2>
        <p id="copy-description" class="mt-1 text-xs leading-5 text-zinc-400">Escolha os idiomas que serão incluídos com {component.label ?? component.name}.</p>
      </div>
      <button type="button" class="rounded px-2 py-1 text-zinc-400 hover:bg-blender-hover hover:text-zinc-100" onclick={onclose} aria-label="Fechar" disabled={copying}>×</button>
    </div>
    <fieldset class="mt-5 space-y-2" disabled={copying}>
      <legend class="flex w-full justify-between gap-3 text-xs font-medium text-zinc-300">Idiomas a incluir <span class="text-[10px] text-zinc-500">{selectedLocales.length} seleccionado(s)</span></legend>
      <div class="grid gap-2 sm:grid-cols-2">
        {#each locales as language (language)}
          {@const selected = selectedLocales.includes(language)}
          <label class="flex cursor-pointer items-center gap-3 rounded-lg border px-3 py-3 {selected ? 'border-blue-500/60 bg-blue-500/10' : 'border-blender-border bg-blender-base hover:border-zinc-500'}">
            <input type="checkbox" value={language} bind:group={selectedLocales} class="h-4 w-4 accent-blue-500" />
            <span class="flex-1 text-sm text-zinc-200">{localeNames[language] ?? language}<span class="block text-[10px] uppercase text-zinc-500">{language}</span></span>
            {#if selected}<span class="text-xs text-blue-400">Incluído</span>{/if}
          </label>
        {/each}
      </div>
    </fieldset>
    {#if errorMessage}<p class="mt-3 text-xs text-rose-400" role="alert">{errorMessage}</p>{/if}
    <div class="mt-5 flex justify-end gap-2">
      <button type="button" class="rounded-md border border-blender-border px-3 py-2 text-xs text-zinc-300 hover:bg-blender-hover" onclick={onclose} disabled={copying}>Cancelar</button>
      <button type="button" class="rounded-md bg-blue-600 px-3 py-2 text-xs font-semibold text-white hover:bg-blue-500 disabled:opacity-50" onclick={copy} disabled={!selectedLocales.length || copying}>{copying ? "A copiar…" : "Copiar para projecto"}</button>
    </div>
  </div>
</div>
