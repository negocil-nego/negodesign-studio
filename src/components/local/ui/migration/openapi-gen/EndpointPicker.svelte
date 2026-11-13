<script lang="ts">
  import type { OpenApiEntity, OpenApiOperation } from "../../../data/migration";
  import EndpointEntityRow from "./EndpointEntityRow.svelte";

  let {
    entities,
    filteredEntities,
    searchQuery = $bindable(""),
    selectedEntityNames,
    selectedEndpointKeys,
    ontoggleentity,
    ontoggleendpoint,
    onselectall,
    onclear,
  }: {
    entities: OpenApiEntity[];
    filteredEntities: OpenApiEntity[];
    searchQuery?: string;
    selectedEntityNames: string[];
    selectedEndpointKeys: string[];
    ontoggleentity: (name: string) => void;
    ontoggleendpoint: (entityName: string, op: OpenApiOperation) => void;
    onselectall: () => void;
    onclear: () => void;
  } = $props();
</script>

<section class="space-y-3">
  <div class="flex flex-wrap items-center justify-between gap-2">
    <div>
      <h3 class="font-semibold text-zinc-800 dark:text-zinc-100">
        1. Selecção de Entidades & Endpoints
      </h3>
      <p class="text-[11px] text-zinc-400">
        Escolha quais endpoints/entidades serão integrados nas páginas AdminCrud. ({selectedEntityNames.length} de {entities.length} seleccionados)
      </p>
    </div>
    <div class="flex items-center gap-2">
      <button
        type="button"
        onclick={onselectall}
        class="rounded border border-blender-border px-2 py-1 text-[10px] font-medium text-blue-400 hover:bg-blender-base transition"
      >
        Seleccionar todas
      </button>
      <button
        type="button"
        onclick={onclear}
        class="rounded border border-blender-border px-2 py-1 text-[10px] font-medium text-zinc-400 hover:bg-blender-base transition"
      >
        Limpar selecção
      </button>
    </div>
  </div>

  <input
    type="search"
    bind:value={searchQuery}
    placeholder="Filtrar por nome, rota, método HTTP ou sumário..."
    class="w-full rounded-lg border border-blender-border bg-blender-base px-3 py-2 text-xs text-zinc-200 placeholder-zinc-500 focus:border-blue-500 focus:outline-none"
  />

  <div class="max-h-60 overflow-y-auto rounded-lg border border-blender-border bg-blender-base/40 divide-y divide-blender-border/40">
    {#each filteredEntities as entity (entity.name)}
      <EndpointEntityRow
        {entity}
        isSelected={selectedEntityNames.includes(entity.name)}
        {selectedEndpointKeys}
        {ontoggleentity}
        {ontoggleendpoint}
      />
    {/each}
  </div>
</section>
