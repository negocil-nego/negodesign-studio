<script lang="ts">
  import {
    entityKindLabels,
    openApiEntities,
    type OpenApiEntityKind,
  } from "../../../../components/local/data/migration";
  import MigrationEmptyState from "../../../../components/local/ui/migration/MigrationEmptyState.svelte";
  import { StyleComponent } from "../../../../components/local/data/style.component";

  type EntityFilter = OpenApiEntityKind | "all";

  let selectedKind = $state<EntityFilter>("all");

  const filters = $derived<EntityFilter[]>([
    "all",
    ...new Set($openApiEntities.map((entity) => entity.kind)),
  ]);
  const filtered = $derived(
    selectedKind === "all"
      ? $openApiEntities
      : $openApiEntities.filter((entity) => entity.kind === selectedKind),
  );

  function count(kind: EntityFilter) {
    return kind === "all"
      ? $openApiEntities.length
      : $openApiEntities.filter((entity) => entity.kind === kind).length;
  }
</script>

<svelte:head>
  <title>OpenApi · Negodesign</title>
  <meta name="description" content="Entidades extraídas da documentação Swagger/OpenAPI." />
</svelte:head>

<div class="mx-auto flex w-full max-w-7xl flex-col gap-6 px-5 py-8 sm:px-8">
  <header>
    <p class="text-xs font-semibold uppercase tracking-[0.18em] text-blue-500">Migração</p>
    <h1 class="mt-2 text-3xl font-semibold tracking-tight text-zinc-800 dark:text-zinc-100">
      Entidades OpenApi
    </h1>
    <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-400">
      {$openApiEntities.length} entidade(s) extraída(s).
    </p>
  </header>

  {#if $openApiEntities.length}
    <nav class="flex flex-wrap gap-2" aria-label="Filtrar por tipo de entidade">
      {#each filters as kind (kind)}
        <button
          type="button"
          aria-pressed={selectedKind === kind}
          onclick={() => (selectedKind = kind)}
          class={`rounded-full border px-3 py-1.5 text-xs font-medium transition ${
            selectedKind === kind
              ? "border-blue-500/50 bg-blue-500/10 text-blue-600 dark:text-blue-300"
              : "border-blender-border bg-blender-card text-zinc-600 hover:border-blue-500/50 dark:text-zinc-300"
          }`}
        >
          {kind === "all" ? "Todas" : entityKindLabels[kind]}
          <span class="ml-1.5 opacity-60">{count(kind)}</span>
        </button>
      {/each}
    </nav>

    <div class="grid gap-4 lg:grid-cols-2">
      {#each filtered as entity (entity.name)}
        <article class={StyleComponent.migration.card}>
          <header class="flex items-center justify-between gap-3 border-b border-blender-border px-4 py-3">
            <h2 class="truncate font-mono text-sm font-semibold text-zinc-800 dark:text-zinc-100" title={entity.name}>
              {entity.name}
            </h2>
            <span class={StyleComponent.migration.badge}>{entityKindLabels[entity.kind]}</span>
          </header>
          {#if entity.fields.length}
            <div class="overflow-x-auto">
              <table class="w-full text-left text-xs">
                <thead class={StyleComponent.migration.cellHead}>
                  <tr>
                    <th class="px-4 py-2 font-semibold">Campo</th>
                    <th class="px-4 py-2 font-semibold">Tipo</th>
                    <th class="px-4 py-2 font-semibold">Obrigatório</th>
                  </tr>
                </thead>
                <tbody>
                  {#each entity.fields as field (field.name)}
                    <tr class="border-t border-blender-border/60">
                      <td class="px-4 py-2 font-mono text-zinc-700 dark:text-zinc-200">{field.name}</td>
                      <td class="px-4 py-2 text-zinc-500">{field.type}</td>
                      <td class="px-4 py-2 text-zinc-500">{field.required ? "Sim" : "Não"}</td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          {:else}
            <p class="px-4 py-3 text-xs text-zinc-500">Sem campos definidos.</p>
          {/if}
        </article>
      {/each}
    </div>
  {:else}
    <MigrationEmptyState
      title="Ainda não há entidades importadas"
      description="Abra «Novo Projecto» no menu lateral, escolha Migração de projecto → OpenApi e indique a rota da documentação Swagger/OpenAPI."
    />
  {/if}
</div>
