<script lang="ts">
  import {
    classifyOpenApiOperation,
    entityKindLabels,
    openApiEntities,
    summarizeEntityCrudPattern,
    type OpenApiCrudRole,
    type OpenApiEntityKind,
  } from "../../../../components/local/data/migration";
  import MigrationEmptyState from "../../../../components/local/ui/migration/MigrationEmptyState.svelte";
  import ApiDocsSection from "../../../../components/local/ui/migration/ApiDocsSection.svelte";
  import { LocalStyleComponent } from "../../../../components/local/data/style.component";
  import OpenApiEntityBuilder from "../../../../components/local/ui/migration/OpenApiEntityBuilder.svelte";
  import OpenApiGeneratorDialog from "../../../../components/local/ui/migration/OpenApiGeneratorDialog.svelte";

  type EntityFilter =
    | OpenApiEntityKind
    | "all"
    | "admin-crud"
    | "restful"
    | "named";

  let selectedKind = $state<EntityFilter>("all");
  let selectedEntity = $state<(typeof $openApiEntities)[number] | null>(null);
  let showConnect = $state(false);
  let showGeneratorDialog = $state(false);
  let checkedEntities = $state<string[]>([]);
  let preselectedEntityNames = $state<string[]>([]);

  const crudRoleLabels: Record<OpenApiCrudRole, string> = {
    list: "GET · Listar (/list, /all)",
    get: "GET · Detalhe (/{id})",
    create: "POST · Criar (/create, /add)",
    update: "PUT/PATCH · Actualizar (/update, /edit, /save)",
    delete: "DELETE · Remover (/delete, /remove)",
  };

  function matchesFilter(
    entity: (typeof $openApiEntities)[number],
    filter: EntityFilter,
  ): boolean {
    if (filter === "all") return true;
    if (filter === "admin-crud") {
      return summarizeEntityCrudPattern(entity).hasCrud;
    }
    if (filter === "restful") {
      return (entity.operations ?? []).some(
        (op) => classifyOpenApiOperation(op).patternStyle === "restful",
      );
    }
    if (filter === "named") {
      return (entity.operations ?? []).some(
        (op) => classifyOpenApiOperation(op).patternStyle === "named",
      );
    }
    return entity.kind === filter;
  }

  const kindFilters = $derived<OpenApiEntityKind[]>([
    ...new Set($openApiEntities.map((entity) => entity.kind)),
  ]);

  const filtered = $derived(
    $openApiEntities.filter((entity) => matchesFilter(entity, selectedKind)),
  );

  function count(filter: EntityFilter) {
    return $openApiEntities.filter((entity) => matchesFilter(entity, filter))
      .length;
  }

  function toggleCheckedEntity(name: string) {
    if (checkedEntities.includes(name)) {
      checkedEntities = checkedEntities.filter((n) => n !== name);
    } else {
      checkedEntities = [...checkedEntities, name];
    }
  }

  function toggleSelectAll() {
    if (checkedEntities.length === $openApiEntities.length) {
      checkedEntities = [];
    } else {
      checkedEntities = $openApiEntities.map((e) => e.name);
    }
  }

  function methodBadgeClass(method: string): string {
    const m = method.toUpperCase();
    if (m === "GET")
      return "border-emerald-500/30 bg-emerald-500/10 text-emerald-400";
    if (m === "POST")
      return "border-blue-500/30 bg-blue-500/10 text-blue-400";
    if (m === "PUT" || m === "PATCH")
      return "border-amber-500/30 bg-amber-500/10 text-amber-400";
    if (m === "DELETE")
      return "border-rose-500/30 bg-rose-500/10 text-rose-400";
    return "border-purple-500/30 bg-purple-500/10 text-purple-400";
  }
</script>

<svelte:head>
  <title>OpenApi · Negodesign</title>
  <meta
    name="description"
    content="Entidades extraídas da documentação Swagger/OpenAPI."
  />
</svelte:head>

<div class="mx-auto flex w-full max-w-7xl flex-col gap-6 px-5 py-8 sm:px-8">
  <header class="flex flex-wrap items-start justify-between gap-4">
    <div>
      <p
        class="text-xs font-semibold uppercase tracking-[0.18em] text-blue-500"
      >
        Migração
      </p>
      <h1
        class="mt-2 text-3xl font-semibold tracking-tight text-zinc-800 dark:text-zinc-100"
      >
        Entidades OpenApi
      </h1>
      <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-400">
        {$openApiEntities.length} entidade(s) extraída(s).
        {#if checkedEntities.length > 0}
          · <span class="font-medium text-emerald-400"
            >{checkedEntities.length} seleccionada(s) para AdminCrud</span
          >
        {/if}
      </p>
    </div>
    <div class="flex flex-wrap items-center gap-2.5">
      {#if $openApiEntities.length}
        <button
          type="button"
          class="rounded-lg border border-blender-border bg-blender-card px-3 py-2 text-xs font-medium text-zinc-200 transition hover:bg-blender-hover"
          onclick={toggleSelectAll}
        >
          {checkedEntities.length === $openApiEntities.length
            ? "Desmarcar todas"
            : "Seleccionar todas"}
        </button>
        <button
          type="button"
          class="rounded-md bg-emerald-600 px-3.5 py-2 text-xs font-semibold text-white shadow-sm hover:bg-emerald-500 transition-colors flex items-center gap-2"
          onclick={() => {
            preselectedEntityNames =
              checkedEntities.length > 0 ? [...checkedEntities] : [];
            showGeneratorDialog = true;
          }}
        >
          <span>◈</span>
          <span>Gerar Frontend (AdminCrud)</span>
          <span class="rounded-full bg-emerald-700/80 px-2 py-0.5 text-[10px]">
            {checkedEntities.length > 0
              ? checkedEntities.length
              : $openApiEntities.length}
          </span>
        </button>
      {/if}
      <button
        type="button"
        class={LocalStyleComponent.migration.primaryButton}
        onclick={() => (showConnect = !showConnect)}
      >
        {showConnect ? "Fechar ligação" : "◈ Ligar / reimportar OpenAPI"}
      </button>
    </div>
  </header>

  {#if showConnect}
    <div
      class="rounded-xl border border-blender-border bg-blender-card p-5 shadow-sm"
    >
      <ApiDocsSection onclose={() => (showConnect = false)} />
    </div>
  {/if}

  {#if $openApiEntities.length}
    <nav class="flex flex-wrap gap-2" aria-label="Filtrar por tipo de entidade ou endpoint">
      {#each [
        { id: "all", label: "Todas" },
        { id: "admin-crud", label: "Com Endpoints AdminCrud" },
        { id: "restful", label: "API RESTful [GET, POST, PUT/PATCH, DELETE]" },
        { id: "named", label: "Nomenclatura (/list, /create, /update, /delete)" },
      ] as tab (tab.id)}
        <button
          type="button"
          aria-pressed={selectedKind === tab.id}
          onclick={() => (selectedKind = tab.id as EntityFilter)}
          class={`rounded-full border px-3 py-1.5 text-xs font-medium transition ${
            selectedKind === tab.id
              ? "border-emerald-500/50 bg-emerald-500/10 text-emerald-400"
              : "border-blender-border bg-blender-card text-zinc-600 hover:border-emerald-500/50 dark:text-zinc-300"
          }`}
        >
          {tab.label}
          <span class="ml-1.5 opacity-60">{count(tab.id as EntityFilter)}</span>
        </button>
      {/each}

      {#each kindFilters as kind (kind)}
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
          {entityKindLabels[kind]}
          <span class="ml-1.5 opacity-60">{count(kind)}</span>
        </button>
      {/each}
    </nav>

    <div class="rounded-xl border border-blender-border/70 bg-blender-card/60 p-3.5 text-[11px] text-zinc-400 space-y-1.5">
      <div class="flex flex-wrap items-center justify-between gap-2">
        <span class="font-semibold text-zinc-200">
          Identificação de Endpoints para `AdminCrud` (RESTful & Nomenclatura de Métodos):
        </span>
        <div class="flex flex-wrap gap-3 text-[10px]">
          <span>
            <span class="font-semibold text-blue-400">DTO / Request</span> → Formulário de criação · Modal Form
          </span>
          <span>
            <span class="font-semibold text-purple-400">Response / Schema</span> → DataTable · Grid · Carousel
          </span>
        </div>
      </div>
      <div class="grid gap-2 sm:grid-cols-4 text-[10px]">
        <div class="rounded border border-emerald-500/20 bg-emerald-500/5 px-2.5 py-1.5">
          <span class="font-semibold text-emerald-400">GET (Listar):</span>
          <span class="ml-1 font-mono text-zinc-400">/list, /all, /find-all, /search, REST GET</span>
        </div>
        <div class="rounded border border-blue-500/20 bg-blue-500/5 px-2.5 py-1.5">
          <span class="font-semibold text-blue-400">POST (Criar):</span>
          <span class="ml-1 font-mono text-zinc-400">/create, /add, /insert, /new, REST POST</span>
        </div>
        <div class="rounded border border-amber-500/20 bg-amber-500/5 px-2.5 py-1.5">
          <span class="font-semibold text-amber-400">PUT/PATCH (Editar):</span>
          <span class="ml-1 font-mono text-zinc-400">/update, /edit, /save, /modify, REST PUT/PATCH</span>
        </div>
        <div class="rounded border border-rose-500/20 bg-rose-500/5 px-2.5 py-1.5">
          <span class="font-semibold text-rose-400">DELETE (Remover):</span>
          <span class="ml-1 font-mono text-zinc-400">/delete, /remove, /destroy, REST DELETE</span>
        </div>
      </div>
    </div>

    <div class="grid gap-4 lg:grid-cols-2">
      {#each filtered as entity (entity.name)}
        {@const isCreation = entity.kind === "dto" || entity.kind === "request"}
        {@const isChecked = checkedEntities.includes(entity.name)}
        {@const crudSummary = summarizeEntityCrudPattern(entity)}
        <article
          class={`${LocalStyleComponent.migration.card} ${
            isChecked ? "ring-1 ring-emerald-500/60" : ""
          }`}
        >
          <header
            class="flex items-center justify-between gap-3 border-b border-blender-border px-4 py-3"
          >
            <label class="flex items-center gap-2.5 min-w-0 cursor-pointer flex-1">
              <input
                type="checkbox"
                checked={isChecked}
                onchange={() => toggleCheckedEntity(entity.name)}
                class="size-4 rounded border-blender-border text-emerald-600"
              />
              <span
                class="shrink-0 rounded px-1.5 py-0.5 text-[9px] font-semibold uppercase tracking-wider {isCreation
                  ? 'bg-blue-500/10 text-blue-400'
                  : 'bg-purple-500/10 text-purple-400'}"
              >
                {isCreation ? "DTO/Request" : "Response"}
              </span>
              <h2
                class="truncate font-mono text-sm font-semibold text-zinc-800 dark:text-zinc-100"
                title={entity.name}
              >
                {entity.name}
              </h2>
            </label>
            <div class="flex items-center gap-1.5 shrink-0">
              <span
                class="rounded border border-emerald-500/30 bg-emerald-500/10 px-2 py-0.5 text-[9px] font-medium text-emerald-300"
                title="Padrão de endpoints identificado para AdminCrud"
              >
                {crudSummary.styleLabel}
              </span>
              <span class={LocalStyleComponent.migration.badge}
                >{entityKindLabels[entity.kind]}</span
              >
            </div>
          </header>
          {#if entity.operations?.length}
            <div
              class="flex flex-col gap-1.5 border-b border-blender-border/60 bg-blender-base/30 px-4 py-2.5"
            >
              <div class="flex items-center justify-between text-[10px] text-zinc-400">
                <span class="font-semibold uppercase tracking-wider">
                  Endpoints Identificados para AdminCrud ({entity.operations.length})
                </span>
                <div class="flex gap-1.5">
                  {#if crudSummary.roles.list.length}
                    <span class="rounded bg-emerald-500/15 px-1.5 py-0.5 text-[9px] font-semibold text-emerald-400">GET/LIST</span>
                  {/if}
                  {#if crudSummary.roles.create.length}
                    <span class="rounded bg-blue-500/15 px-1.5 py-0.5 text-[9px] font-semibold text-blue-400">POST/CREATE</span>
                  {/if}
                  {#if crudSummary.roles.update.length}
                    <span class="rounded bg-amber-500/15 px-1.5 py-0.5 text-[9px] font-semibold text-amber-400">PUT·PATCH/UPDATE</span>
                  {/if}
                  {#if crudSummary.roles.delete.length}
                    <span class="rounded bg-rose-500/15 px-1.5 py-0.5 text-[9px] font-semibold text-rose-400">DELETE</span>
                  {/if}
                </div>
              </div>
              <div class="flex flex-wrap gap-1.5">
                {#each entity.operations as operation (`${operation.method}-${operation.path}`)}
                  {@const info = classifyOpenApiOperation(operation)}
                  <span
                    class="inline-flex items-center gap-1.5 rounded border px-2 py-1 font-mono text-[10px] {methodBadgeClass(operation.method)}"
                    title={`${crudRoleLabels[info.crudRole]} (${info.patternStyle === "named" ? "Nomenclatura de Método" : "API RESTful"})`}
                  >
                    <span class="font-bold">{operation.method}</span>
                    <span>{operation.path}</span>
                    <span class="rounded bg-black/30 px-1 py-0.2 text-[8px] uppercase tracking-wider opacity-85">
                      {info.crudRole} · {info.patternStyle === "named" ? " método" : "rest"}
                    </span>
                  </span>
                {/each}
              </div>
            </div>
          {/if}
          {#if entity.fields.length}
            <div class="overflow-x-auto">
              <table class="w-full text-left text-xs">
                <thead class={LocalStyleComponent.migration.cellHead}>
                  <tr>
                    <th class="px-4 py-2 font-semibold">Campo</th>
                    <th class="px-4 py-2 font-semibold">Tipo</th>
                    <th class="px-4 py-2 font-semibold">Obrigatório</th>
                  </tr>
                </thead>
                <tbody>
                  {#each entity.fields as field (field.name)}
                    <tr class="border-t border-blender-border/60">
                      <td
                        class="px-4 py-2 font-mono text-zinc-700 dark:text-zinc-200"
                        >{field.name}</td
                      >
                      <td class="px-4 py-2 text-zinc-500">{field.type}</td>
                      <td class="px-4 py-2 text-zinc-500"
                        >{field.required ? "Sim" : "Não"}</td
                      >
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          {:else}
            <p class="px-4 py-3 text-xs text-zinc-500">Sem campos definidos.</p>
          {/if}
          <footer
            class="flex items-center justify-between gap-3 border-t border-blender-border/60 px-4 py-3"
          >
            <p class="text-[10px] text-zinc-500">
              {#if isCreation}
                Componentes: <span class="text-blue-400">Form · ModalForm</span>
              {:else}
                Componentes: <span class="text-purple-400"
                  >DataTable · Grid · Carousel · GridCarousel</span
                >
              {/if}
            </p>
            <div class="flex items-center gap-2">
              <button
                type="button"
                class="rounded-md border border-emerald-500/30 bg-emerald-500/10 px-2.5 py-1.5 text-[11px] font-semibold text-emerald-400 hover:bg-emerald-500/20 transition-colors"
                onclick={() => {
                  preselectedEntityNames = [entity.name];
                  showGeneratorDialog = true;
                }}
              >
                ◈ Gerar AdminCrud
              </button>
              <button
                type="button"
                class="rounded-md bg-blue-600 px-3 py-1.5 text-[11px] font-semibold text-white hover:bg-blue-500 transition-colors"
                onclick={() => (selectedEntity = entity)}
              >
                Componentizar {isCreation ? "formulário" : "apresentação"}
              </button>
            </div>
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
  <OpenApiEntityBuilder
    entity={selectedEntity}
    onclose={() => (selectedEntity = null)}
  />
{/if}

{#if showGeneratorDialog}
  <OpenApiGeneratorDialog
    entities={$openApiEntities}
    initialSelectedEntityNames={preselectedEntityNames}
    onclose={() => (showGeneratorDialog = false)}
  />
{/if}
