<script lang="ts">
  import { openApiEntities } from "../../../../components/local/data/migration";
  import { countEntities, distinctKinds, matchesFilter, type EntityFilter } from "../../../../components/local/ui/migration/openapi-page/filters";
  import MigrationEmptyState from "../../../../components/local/ui/migration/MigrationEmptyState.svelte";
  import ApiDocsSection from "../../../../components/local/ui/migration/ApiDocsSection.svelte";
  import { LocalStyleComponent } from "../../../../components/local/data/style.component";
  import OpenApiEntityBuilder from "../../../../components/local/ui/migration/OpenApiEntityBuilder.svelte";
  import OpenApiGeneratorDialog from "../../../../components/local/ui/migration/OpenApiGeneratorDialog.svelte";
  import OpenApiFilterTabs from "../../../../components/local/ui/migration/openapi-page/OpenApiFilterTabs.svelte";
  import OpenApiCrudLegend from "../../../../components/local/ui/migration/openapi-page/OpenApiCrudLegend.svelte";
  import OpenApiEntityCard from "../../../../components/local/ui/migration/openapi-page/OpenApiEntityCard.svelte";

  let selectedKind = $state<EntityFilter>("all");
  let selectedEntity = $state<(typeof $openApiEntities)[number] | null>(null);
  let showConnect = $state(false);
  let showGeneratorDialog = $state(false);
  let checkedEntities = $state<string[]>([]);
  let preselectedEntityNames = $state<string[]>([]);

  const kindFilters = $derived(distinctKinds($openApiEntities));
  const filtered = $derived($openApiEntities.filter((entity) => matchesFilter(entity, selectedKind)));
  const count = (filter: EntityFilter) => countEntities($openApiEntities, filter);

  function toggleCheckedEntity(name: string) {
    checkedEntities = checkedEntities.includes(name)
      ? checkedEntities.filter((n) => n !== name)
      : [...checkedEntities, name];
  }

  function toggleSelectAll() {
    checkedEntities =
      checkedEntities.length === $openApiEntities.length
        ? []
        : $openApiEntities.map((e) => e.name);
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
      <h1 class="mt-2 text-3xl font-semibold tracking-tight text-zinc-800 dark:text-zinc-100">Entidades OpenApi</h1>
      <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-400">
        {$openApiEntities.length} entidade(s) extraída(s).
        {#if checkedEntities.length > 0}
          · <span class="font-medium text-emerald-400">{checkedEntities.length} seleccionada(s) para AdminCrud</span>
        {/if}
      </p>
    </div>
    <div class="flex flex-wrap items-center gap-2.5">
      {#if $openApiEntities.length}
        <button type="button" class="rounded-lg border border-blender-border bg-blender-card px-3 py-2 text-xs font-medium text-zinc-200 transition hover:bg-blender-hover" onclick={toggleSelectAll}>{checkedEntities.length === $openApiEntities.length ? "Desmarcar todas" : "Seleccionar todas"}</button>
        <button type="button" class="rounded-md bg-emerald-600 px-3.5 py-2 text-xs font-semibold text-white shadow-sm hover:bg-emerald-500 transition-colors flex items-center gap-2" onclick={() => { preselectedEntityNames = checkedEntities.length > 0 ? [...checkedEntities] : []; showGeneratorDialog = true; }}>
          <span>◈</span>
          <span>Gerar Frontend (AdminCrud)</span>
          <span class="rounded-full bg-emerald-700/80 px-2 py-0.5 text-[10px]">{checkedEntities.length > 0 ? checkedEntities.length : $openApiEntities.length}</span>
        </button>
      {/if}
      <button type="button" class={LocalStyleComponent.migration.primaryButton} onclick={() => (showConnect = !showConnect)}>{showConnect ? "Fechar ligação" : "◈ Ligar / reimportar OpenAPI"}</button>
    </div>
  </header>

  {#if showConnect}
    <div class="rounded-xl border border-blender-border bg-blender-card p-5 shadow-sm">
      <ApiDocsSection onclose={() => (showConnect = false)} />
    </div>
  {/if}

  {#if $openApiEntities.length}
    <OpenApiFilterTabs selected={selectedKind} {kindFilters} {count} onselect={(filter) => (selectedKind = filter)} />
    <OpenApiCrudLegend />

    <div class="grid gap-4 lg:grid-cols-2">
      {#each filtered as entity (entity.name)}
        <OpenApiEntityCard {entity} isChecked={checkedEntities.includes(entity.name)}
          onToggle={() => toggleCheckedEntity(entity.name)} onComponentize={() => (selectedEntity = entity)}
          onGenerate={() => { preselectedEntityNames = [entity.name]; showGeneratorDialog = true; }} />
      {/each}
    </div>
  {:else if !showConnect}
    <MigrationEmptyState title="Ainda não há entidades importadas" description="Use o botão «Ligar / reimportar OpenAPI» acima para introduzir a rota Swagger (ex: http://localhost:8080/v3/api-docs) ou importar um ficheiro JSON/YAML." />
  {/if}
</div>

{#if selectedEntity}
  <OpenApiEntityBuilder entity={selectedEntity} onclose={() => (selectedEntity = null)} />
{/if}

{#if showGeneratorDialog}
  <OpenApiGeneratorDialog entities={$openApiEntities} initialSelectedEntityNames={preselectedEntityNames} onclose={() => (showGeneratorDialog = false)} />
{/if}
