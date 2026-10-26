<script lang="ts">
  import { componentCatalog, type ComponentDefinition } from "../../../data/components";
  import { activeProject } from "../../../data/projects";
  import { copyComponentToActiveProject } from "../../../data/component-copy";
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
        "input", "input-text", "input-email", "input-phone", "input-phone-flag",
        "input-label", "input-password", "input-date", "input-time", "input-number",
        "input-age"
      ],
      type: "atomic"
    },
    {
      label: "Atoms · Control",
      badge: "Control",
      description: "Controlos de selecção e alternância.",
      ids: ["select", "textarea", "toggle", "radio", "checkbox"],
      type: "atomic"
    },
    {
      label: "Typography",
      badge: "Atom",
      description: "Elementos de texto e rótulo.",
      ids: ["label"],
      type: "atomic"
    },
    {
      label: "Molecules",
      badge: "Molecule",
      description: "Componentes compostos por múltiplos átomos.",
      ids: ["country-popover"],
      type: "molecule"
    },
    {
      label: "Molecules · Card",
      badge: "Molecule",
      description: "Cartões compostos por átomos e sub-cartões reutilizáveis.",
      ids: [
        "card-media", "card-profile", "card-product", "card", "card-image",
        "card-title", "card-description", "card-price", "card-rating",
        "card-tags", "card-logo", "card-phone", "card-video",
        "card-thumbnail-video", "card-min-file", "card-button",
        "card-cart-button", "card-favorite-button", "card-actions-button"
      ],
      type: "molecule"
    },
  ];

  const catalogById = Object.fromEntries(componentCatalog.map((c) => [c.id, c]));
  const components = componentCatalog;
  let copyingId = $state<string | null>(null);
  let copyMessage = $state("");

  async function copyComponent(component: ComponentDefinition) {
    if (!component.dependence) return;
    copyingId = component.id;
    copyMessage = "";
    try {
      const result = await copyComponentToActiveProject(component.id);
      copyMessage = `${component.label ?? component.name}: ${result.copiedFiles} ficheiro(s) e ${result.exportedComponents} componente(s) disponíveis em ${result.project.name}/negodesign.`;
    } catch (error) {
      copyMessage = error instanceof Error ? error.message : String(error);
    } finally {
      copyingId = null;
    }
  }
</script>

<main class="space-y-8">
  <!-- ── Header ──────────────────────────────────────────────────────────── -->
  <header class="flex items-end justify-between gap-4">
    <div>
      <p class="text-xs font-semibold uppercase tracking-[0.16em] text-blue-500">Project UI</p>
      <h1 class="mt-1 text-2xl font-semibold text-zinc-100">Componentes</h1>
      <p class="mt-1 text-sm text-zinc-400">Pré-visualização dos componentes existentes neste projecto.</p>
    </div>
    <span class="shrink-0 rounded-full border border-blender-border bg-blender-card px-3 py-1 text-xs font-medium text-zinc-400">
      {components.length} componentes
    </span>
  </header>

  <!-- ── Active project bar ──────────────────────────────────────────────── -->
  <section class="flex flex-wrap items-center justify-between gap-3 rounded-lg border border-blender-border bg-blender-card px-4 py-3">
    <div class="min-w-0">
      <p class="text-[10px] font-semibold uppercase tracking-wider text-zinc-500">Linguagem do projecto activo</p>
      {#if $activeProject}
        <p class="mt-1 truncate text-sm font-semibold text-zinc-100">
          {$activeProject.language}
          <span class="font-normal text-zinc-400">· {$activeProject.name}</span>
        </p>
      {:else}
        <p class="mt-1 text-sm text-zinc-400">Escolha um projecto no Inspector para activar a cópia de componentes.</p>
      {/if}
    </div>
    {#if $activeProject}
      <span class="rounded-full border border-blue-500/30 bg-blue-500/10 px-3 py-1 text-xs font-medium text-blue-400">{$activeProject.language}</span>
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
            <p class="text-[10px] font-semibold uppercase tracking-wider text-zinc-500">{group.badge}</p>
            <h2 class="text-base font-semibold text-zinc-100">{group.label}</h2>
            <p class="mt-0.5 text-xs text-zinc-500">{group.description}</p>
          </div>
          <span class="shrink-0 rounded-full border border-blender-border bg-blender-base px-2.5 py-0.5 text-[10px] font-medium text-zinc-500">{groupItems.length}</span>
        </div>

        {#if group.type === "atomic"}
          <AtomicComponentScreen components={groupItems} {copyingId} onCopy={copyComponent} />
        {:else}
          <MoleculeComponentScreen components={groupItems} {copyingId} onCopy={copyComponent} />
        {/if}
      </section>
    {/if}
  {/each}
</main>
