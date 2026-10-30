<script lang="ts">
  import {
    componentCatalog,
    type ComponentDefinition,
  } from "../../../data/components";
  import { activeProject } from "../../../data/projects";
  import { copyComponentToActiveProject } from "../../../data/component-copy";
  import { locales } from "../../../../../project/data/i18n";
  import AtomicComponentScreen from "./AtomicComponentScreen.svelte";
  import MoleculeComponentScreen from "./MoleculeComponentScreen.svelte";

  // ─── Types & helpers ────────────────────────────────────────────────────────

  type Group = {
    label: string;
    badge: string;
    description: string;
    ids: string[];
    type: "atomic" | "molecule";
  };

  /** Logical grouping for display. */
  const screenGroups: Group[] = [
    {
      label: "Atoms · Input",
      badge: "Atom",
      description: "Campos de entrada de texto e dados estruturados.",
      ids: [
        "input",
        "input-text",
        "input-username",
        "input-username-or-email",
        "input-username-or-phone",
        "input-otp",
        "input-email",
        "input-phone",
        "input-phone-flag",
        "input-label",
        "input-password",
        "input-date",
        "input-time",
        "input-number",
        "input-age",
      ],
      type: "atomic",
    },
    {
      label: "Atoms · Control",
      badge: "Control",
      description: "Controlos de selecção e alternância.",
      ids: ["select", "textarea", "toggle", "radio", "checkbox"],
      type: "atomic",
    },
    {
      label: "Typography",
      badge: "Atom",
      description: "Elementos de texto e rótulo.",
      ids: ["label"],
      type: "atomic",
    },
    {
      label: "Molecules",
      badge: "Molecule",
      description: "Componentes compostos por múltiplos átomos.",
      ids: ["country-popover"],
      type: "molecule",
    },
    {
      label: "Organisms · Menu",
      badge: "Organism",
      description: "Menus de navegação compostos por logótipo, itens e acções.",
      ids: ["menu-simple"],
      type: "molecule",
    },
    {
      label: "Molecules · Card",
      badge: "Molecule",
      description: "Cartões compostos por átomos e sub-cartões reutilizáveis.",
      ids: [
        "card-media",
        "card-profile",
        "card-product",
        "card",
        "card-image",
        "card-title",
        "card-description",
        "card-price",
        "card-rating",
        "card-tags",
        "card-logo",
        "card-phone",
        "card-video",
        "card-thumbnail-video",
        "card-min-file",
        "card-button",
        "card-cart-button",
        "card-favorite-button",
        "card-actions-button",
      ],
      type: "molecule",
    },
  ];

  const catalogById = Object.fromEntries(
    componentCatalog.map((c) => [c.id, c]),
  );
  const components = componentCatalog;
  let copyingId = $state<string | null>(null);
  let copyMessage = $state("");
  let componentAwaitingCopy = $state<ComponentDefinition | null>(null);
  let selectedLocales = $state<string[]>([]);

  const localeNames: Record<string, string> = {
    en: "English",
    pt: "Português",
  };

  function copyComponent(component: ComponentDefinition) {
    if (!component.dependence) return;
    if (!$activeProject) {
      copyMessage = "Seleccione primeiro um projecto activo no Inspector.";
      return;
    }
    componentAwaitingCopy = component;
    selectedLocales = [...locales];
    copyMessage = "";
  }

  async function confirmCopy() {
    const component = componentAwaitingCopy;
    if (!component?.dependence || !selectedLocales.length) return;
    copyingId = component.id;
    copyMessage = "";
    try {
      const result = await copyComponentToActiveProject(component.id, {
        locales: selectedLocales,
      });
      const languages = selectedLocales
        .map((language) => localeNames[language] ?? language)
        .join(", ");
      copyMessage = `${component.label ?? component.name}: ${result.copiedFiles} ficheiro(s), ${result.exportedComponents} componente(s) e idiomas ${languages} disponíveis em ${result.project.name}/negodesign.`;
      componentAwaitingCopy = null;
    } catch (error) {
      copyMessage = error instanceof Error ? error.message : String(error);
    } finally {
      copyingId = null;
    }
  }

  function closeCopyDialog() {
    if (!copyingId) componentAwaitingCopy = null;
  }
</script>

<main class="w-full min-w-0 space-y-8">
  <!-- ── Header ──────────────────────────────────────────────────────────── -->
  <header class="flex items-end justify-between gap-4">
    <div>
      <p
        class="text-xs font-semibold uppercase tracking-[0.16em] text-blue-500"
      >
        Project UI
      </p>
      <h1 class="mt-1 text-2xl font-semibold text-zinc-100">Componentes</h1>
      <p class="mt-1 text-sm text-zinc-400">
        Pré-visualização dos componentes existentes neste projecto.
      </p>
    </div>
    <span
      class="shrink-0 rounded-full border border-blender-border bg-blender-card px-3 py-1 text-xs font-medium text-zinc-400"
    >
      {components.length} componentes
    </span>
  </header>

  <!-- ── Active project bar ──────────────────────────────────────────────── -->
  <section
    class="flex flex-wrap items-center justify-between gap-3 rounded-lg border border-blender-border bg-blender-card px-4 py-3"
  >
    <div class="min-w-0">
      <p
        class="text-[10px] font-semibold uppercase tracking-wider text-zinc-500"
      >
        Linguagem do projecto activo
      </p>
      {#if $activeProject}
        <p class="mt-1 truncate text-sm font-semibold text-zinc-100">
          {$activeProject.language}
          <span class="font-normal text-zinc-400">· {$activeProject.name}</span>
        </p>
      {:else}
        <p class="mt-1 text-sm text-zinc-400">
          Escolha um projecto no Inspector para activar a cópia de componentes.
        </p>
      {/if}
    </div>
    {#if $activeProject}
      <span
        class="rounded-full border border-blue-500/30 bg-blue-500/10 px-3 py-1 text-xs font-medium text-blue-400"
        >{$activeProject.language}</span
      >
    {/if}
  </section>

  {#if copyMessage}
    <p class="text-xs text-zinc-400" role="status">{copyMessage}</p>
  {/if}

  <!-- ── Component groups ────────────────────────────────────────────────── -->
  {#each screenGroups as group}
    {@const groupItems = group.ids.map((id) => catalogById[id]).filter(Boolean)}
    {#if groupItems.length}
      <section class="space-y-4">
        <!-- Group header -->
        <div class="flex items-end gap-3 border-b border-blender-border pb-2">
          <div class="min-w-0 flex-1">
            <p
              class="text-[10px] font-semibold uppercase tracking-wider text-zinc-500"
            >
              {group.badge}
            </p>
            <h2 class="text-base font-semibold text-zinc-100">{group.label}</h2>
            <p class="mt-0.5 text-xs text-zinc-500">{group.description}</p>
          </div>
          <span
            class="shrink-0 rounded-full border border-blender-border bg-blender-base px-2.5 py-0.5 text-[10px] font-medium text-zinc-500"
            >{groupItems.length}</span
          >
        </div>

        {#if group.type === "atomic"}
          <AtomicComponentScreen
            components={groupItems}
            {copyingId}
            onCopy={copyComponent}
          />
        {:else}
          <MoleculeComponentScreen
            components={groupItems}
            {copyingId}
            onCopy={copyComponent}
          />
        {/if}
      </section>
    {/if}
  {/each}
</main>

{#if componentAwaitingCopy}
  <div
    class="fixed inset-0 z-[100] flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm"
    role="presentation"
    onclick={(event) => {
      if (event.target === event.currentTarget) closeCopyDialog();
    }}
    onkeydown={(event) => {
      if (event.key === "Escape") closeCopyDialog();
    }}
  >
    <section
      class="w-full max-w-md rounded-xl border border-blender-border bg-blender-card p-5 shadow-2xl"
      role="dialog"
      aria-modal="true"
      aria-labelledby="component-copy-title"
      aria-describedby="component-copy-description"
      tabindex="-1"
    >
      <div class="flex items-start justify-between gap-4">
        <div>
          <h2 id="component-copy-title" class="text-base font-semibold text-zinc-100">
            Internacionalizar componente
          </h2>
          <p id="component-copy-description" class="mt-1 text-xs leading-5 text-zinc-400">
            Escolha os idiomas que serão incluídos com
            {componentAwaitingCopy.label ?? componentAwaitingCopy.name}.
          </p>
        </div>
        <button
          type="button"
          class="rounded px-2 py-1 text-zinc-400 hover:bg-blender-hover hover:text-zinc-100"
          onclick={closeCopyDialog}
          aria-label="Fechar"
          disabled={!!copyingId}
        >
          ×
        </button>
      </div>

      <fieldset class="mt-5 space-y-2" disabled={!!copyingId}>
        <legend class="flex w-full items-center justify-between gap-3 text-xs font-medium text-zinc-300">
          <span>
            Idiomas a incluir
          </span>
          <span class="text-[10px] text-zinc-500">
            {selectedLocales.length} selecionado(s)
          </span>
        </legend>
        <div class="grid grid-cols-1 gap-2 sm:grid-cols-2">
          {#each locales as language (language)}
            {@const selected = selectedLocales.includes(language)}
            <label
              class="flex cursor-pointer items-center gap-3 rounded-lg border px-3 py-3 transition-colors {selected
                ? 'border-blue-500/60 bg-blue-500/10'
                : 'border-blender-border bg-blender-base hover:border-zinc-500'}"
            >
              <input
                type="checkbox"
                value={language}
                bind:group={selectedLocales}
                class="h-4 w-4 accent-blue-500"
              />
              <span class="min-w-0 flex-1">
                <span class="block text-sm font-medium text-zinc-200">
                  {localeNames[language] ?? language}
                </span>
                <span class="block text-[10px] uppercase tracking-wider text-zinc-500">
                  {language}
                </span>
              </span>
              {#if selected}
                <span class="text-xs font-medium text-blue-400">Incluído</span>
              {/if}
            </label>
          {/each}
        </div>
      </fieldset>
      {#if copyMessage}
        <p class="mt-3 text-xs text-rose-400" role="alert">{copyMessage}</p>
      {/if}

      <div class="mt-5 flex justify-end gap-2">
        <button
          type="button"
          class="rounded-md border border-blender-border px-3 py-2 text-xs text-zinc-300 transition-colors hover:bg-blender-hover"
          onclick={closeCopyDialog}
          disabled={!!copyingId}
        >
          Cancelar
        </button>
        <button
          type="button"
          class="rounded-md bg-blue-600 px-3 py-2 text-xs font-semibold text-white transition-colors hover:bg-blue-500 disabled:cursor-not-allowed disabled:opacity-50"
          onclick={confirmCopy}
          disabled={!selectedLocales.length || !!copyingId}
        >
          {copyingId ? "A copiar…" : "Copiar para projecto"}
        </button>
      </div>
    </section>
  </div>
{/if}
