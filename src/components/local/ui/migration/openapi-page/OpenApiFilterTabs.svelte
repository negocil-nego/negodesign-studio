<script lang="ts">
  import { entityKindLabels, type OpenApiEntityKind } from "../../../data/migration";
  import type { EntityFilter } from "./filters";

  let {
    selected,
    kindFilters,
    count,
    onselect,
  }: {
    selected: EntityFilter;
    kindFilters: OpenApiEntityKind[];
    count: (filter: EntityFilter) => number;
    onselect: (filter: EntityFilter) => void;
  } = $props();

  const tabs: { id: EntityFilter; label: string }[] = [
    { id: "all", label: "Todas" },
    { id: "admin-crud", label: "Com Endpoints AdminCrud" },
    { id: "restful", label: "API RESTful [GET, POST, PUT/PATCH, DELETE]" },
    { id: "named", label: "Nomenclatura (/list, /create, /update, /delete)" },
  ];
</script>

<nav class="flex flex-wrap gap-2" aria-label="Filtrar por tipo de entidade ou endpoint">
  {#each tabs as tab (tab.id)}
    <button
      type="button"
      aria-pressed={selected === tab.id}
      onclick={() => onselect(tab.id)}
      class={`rounded-full border px-3 py-1.5 text-xs font-medium transition ${
        selected === tab.id
          ? "border-emerald-500/50 bg-emerald-500/10 text-emerald-400"
          : "border-blender-border bg-blender-card text-zinc-600 hover:border-emerald-500/50 dark:text-zinc-300"
      }`}
    >
      {tab.label}
      <span class="ml-1.5 opacity-60">{count(tab.id)}</span>
    </button>
  {/each}

  {#each kindFilters as kind (kind)}
    <button
      type="button"
      aria-pressed={selected === kind}
      onclick={() => onselect(kind)}
      class={`rounded-full border px-3 py-1.5 text-xs font-medium transition ${
        selected === kind
          ? "border-blue-500/50 bg-blue-500/10 text-blue-600 dark:text-blue-300"
          : "border-blender-border bg-blender-card text-zinc-600 hover:border-blue-500/50 dark:text-zinc-300"
      }`}
    >
      {entityKindLabels[kind]}
      <span class="ml-1.5 opacity-60">{count(kind)}</span>
    </button>
  {/each}
</nav>
