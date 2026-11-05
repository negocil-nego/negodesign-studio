<script lang="ts">
  import {
    entityKindLabels,
    openApiEntities,
    type OpenApiEntityKind,
  } from "../../../../components/local/data/migration";
  import MigrationEmptyState from "../../../../components/local/ui/migration/MigrationEmptyState.svelte";
  import ApiDocsSection from "../../../../components/local/ui/migration/ApiDocsSection.svelte";
  import { StyleComponent } from "../../../../components/local/data/style.component";
  import OpenApiEntityBuilder from "../../../../components/local/ui/migration/OpenApiEntityBuilder.svelte";

  type EntityFilter = OpenApiEntityKind | "all";

  let selectedKind = $state<EntityFilter>("all");
  let selectedEntity = $state<(typeof $openApiEntities)[number] | null>(null);
  let showConnect = $state(false);

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
  <header class="flex flex-wrap items-start justify-between gap-4">
    <div>
      <p class="text-xs font-semibold uppercase tracking-[0.18em] text-blue-500">Migração</p>
      <h1 class="mt-2 text-3xl font-semibold tracking-tight text-zinc-800 dark:text-zinc-100">
        Entidades OpenApi
      </h1>
      <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-400">
        {$openApiEntities.length} entidade(s) extraída(s).
      </p>
    </div>
    <button
      type="button"
      class={StyleComponent.migration.primaryButton}
      onclick={() => (showConnect = !showConnect)}
    >
      {showConnect ? "Fechar ligação" : "◈ Ligar / reimportar OpenAPI"}
    </button>
  </header>

  {#if showConnect}
    <div class="rounded-xl border border-blender-border bg-blender-card p-5 shadow-sm">
      <ApiDocsSection onclose={() => (showConnect = false)} />
    </div>
  {/if}

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

    <div class="flex flex-wrap gap-4 text-[10px] text-zinc-500">
      <span>
        <span class="font-semibold text-blue-400">DTO / Request</span> →
        Formulário de criação · Modal Form
      </span>
      <span>
        <span class="font-semibold text-purple-400">Response / Schema</span> →
        DataTable · Grid · Carousel · Grid-Carousel
      </span>
    </div>

    <div class="grid gap-4 lg:grid-cols-2">
      {#each filtered as entity (entity.name)}
        {@const isCreation = entity.kind === "dto" || entity.kind === "request"}
        <article class={StyleComponent.migration.card}>
          <header class="flex items-center justify-between gap-3 border-b border-blender-border px-4 py-3">
            <div class="flex items-center gap-2 min-w-0">
              <span
                class="shrink-0 rounded px-1.5 py-0.5 text-[9px] font-semibold uppercase tracking-wider {isCreation
                  ? 'bg-blue-500/10 text-blue-400'
                  : 'bg-purple-500/10 text-purple-400'}"
              >
                {isCreation ? "DTO/Request" : "Response"}
              </span>
              <h2 class="truncate font-mono text-sm font-semibold text-zinc-800 dark:text-zinc-100" title={entity.name}>
                {entity.name}
              </h2>
            </div>
            <span class={StyleComponent.migration.badge}>{entityKindLabels[entity.kind]}</span>
          </header>
          {#if entity.operations?.length}
            <div class="flex flex-wrap gap-1.5 border-b border-blender-border/60 px-4 py-2">
              {#each entity.operations as operation (`${operation.method}-${operation.path}`)}
                <span class="rounded bg-blender-base px-2 py-1 font-mono text-[10px] text-zinc-500">{operation.method} {operation.path}</span>
              {/each}
            </div>
          {/if}
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
          <footer class="flex items-center justify-between gap-3 border-t border-blender-border/60 px-4 py-3">
            <p class="text-[10px] text-zinc-500">
              {#if isCreation}
                Componentes: <span class="text-blue-400">Form · ModalForm</span>
              {:else}
                Componentes: <span class="text-purple-400">DataTable · Grid · Carousel · GridCarousel</span>
              {/if}
            </p>
            <button
              type="button"
              class="rounded-md bg-blue-600 px-3 py-2 text-[11px] font-semibold text-white hover:bg-blue-500 transition-colors"
              onclick={() => (selectedEntity = entity)}
            >
              Componentizar {isCreation ? "formulário" : "apresentação"}
            </button>
          </footer>
        </article>
      {/each}
    </div>
  {:else if !showConnect}
    <MigrationEmptyState
      title="Ainda não há entidades importadas"
      description="Use o botão «Ligar / reimportar OpenAPI» acima para introduzir a rota Swagger (ex: http://localhost:8080/v3/api-docs) ou importar um ficheiro JSON/YAML."
    />
  {/if}
</div>

{#if selectedEntity}
  <OpenApiEntityBuilder entity={selectedEntity} onclose={() => (selectedEntity = null)} />
{/if}
