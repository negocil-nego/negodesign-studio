<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { LocalStyleComponent } from "../../data/style.component";
  import { activeProject } from "../../data/projects";
  import { bootswatchUi } from "../../data/bootswatch-ui.svelte";
  import {
    classifyOpenApiOperation,
    generateOpenApiFrontend,
    summarizeEntityCrudPattern,
    type ApiCallPattern,
    type FrontendFramework,
    type HttpClientKind,
    type OpenApiEndpointSelection,
    type OpenApiEntity,
    type OpenApiFrontendCodegenResult,
    type OpenApiOperation,
  } from "../../data/migration";

  let {
    entities = [],
    initialSelectedEntityNames = [],
    onclose,
  }: {
    entities: OpenApiEntity[];
    initialSelectedEntityNames?: string[];
    onclose: () => void;
  } = $props();

  function endpointKey(entityName: string, method: string, path: string): string {
    return `${entityName}::${method.toUpperCase()}::${path}`;
  }

  function defaultOperationsForEntity(entity: OpenApiEntity): OpenApiOperation[] {
    if (entity.operations && entity.operations.length > 0) {
      return entity.operations;
    }
    const base = entity.name
      .replace(/(Response|Request|Dto|Model|Entity|Schema)$/i, "")
      .replace(/([a-z0-9])([A-Z])/g, "$1-$2")
      .toLowerCase();
    const slug = base || entity.name.toLowerCase();
    return [
      { method: "GET", path: `/api/${slug}`, crudRole: "list", patternStyle: "restful" },
      { method: "POST", path: `/api/${slug}`, crudRole: "create", patternStyle: "restful" },
      { method: "PUT", path: `/api/${slug}/{id}`, crudRole: "update", patternStyle: "restful" },
      { method: "DELETE", path: `/api/${slug}/{id}`, crudRole: "delete", patternStyle: "restful" },
    ];
  }

  // svelte-ignore state_referenced_locally
  const initialEntities =
    initialSelectedEntityNames.length > 0
      ? [...initialSelectedEntityNames]
      : entities.map((e) => e.name);

  let selectedEntityNames = $state<string[]>(initialEntities);

  // svelte-ignore state_referenced_locally
  let selectedEndpointKeys = $state<string[]>(
    entities
      .filter((e) => initialEntities.includes(e.name))
      .flatMap((e) =>
        defaultOperationsForEntity(e).map((op) =>
          endpointKey(e.name, op.method, op.path),
        ),
      ),
  );

  let searchQuery = $state("");
  let frontendFramework = $state<FrontendFramework>("sveltekit");
  let frontendPath = $state($activeProject?.path ?? "");
  let apiCallPattern = $state<ApiCallPattern>("bff");
  let httpClient = $state<HttpClientKind>("native");
  let useTanstackQuery = $state<boolean>(true);
  
  // svelte-ignore state_referenced_locally
  const defaultServerUrl = entities.find((e) => e.serverUrl)?.serverUrl ?? "http://localhost:8080";
  let backendBaseUrl = $state(defaultServerUrl);

  let generating = $state(false);
  let errorMessage = $state("");
  let result = $state<OpenApiFrontendCodegenResult | null>(null);

  const filteredEntities = $derived(
    entities.filter((e) => {
      if (!searchQuery.trim()) return true;
      const q = searchQuery.toLowerCase();
      return (
        e.name.toLowerCase().includes(q) ||
        e.kind.toLowerCase().includes(q) ||
        e.operations.some(
          (op) =>
            op.path.toLowerCase().includes(q) ||
            op.method.toLowerCase().includes(q) ||
            (op.summary && op.summary.toLowerCase().includes(q)),
        )
      );
    }),
  );

  function toggleEntity(name: string) {
    const entity = entities.find((e) => e.name === name);
    const ops = entity ? defaultOperationsForEntity(entity) : [];
    const keys = ops.map((op) => endpointKey(name, op.method, op.path));

    if (selectedEntityNames.includes(name)) {
      selectedEntityNames = selectedEntityNames.filter((n) => n !== name);
      selectedEndpointKeys = selectedEndpointKeys.filter((k) => !keys.includes(k));
    } else {
      selectedEntityNames = [...selectedEntityNames, name];
      const next = new Set([...selectedEndpointKeys, ...keys]);
      selectedEndpointKeys = [...next];
    }
  }

  function toggleEndpoint(entityName: string, op: OpenApiOperation) {
    const key = endpointKey(entityName, op.method, op.path);
    if (selectedEndpointKeys.includes(key)) {
      selectedEndpointKeys = selectedEndpointKeys.filter((k) => k !== key);
      const entity = entities.find((e) => e.name === entityName);
      const remaining = (entity ? defaultOperationsForEntity(entity) : []).some(
        (o) => selectedEndpointKeys.includes(endpointKey(entityName, o.method, o.path)),
      );
      if (!remaining) {
        selectedEntityNames = selectedEntityNames.filter((n) => n !== entityName);
      }
    } else {
      selectedEndpointKeys = [...selectedEndpointKeys, key];
      if (!selectedEntityNames.includes(entityName)) {
        selectedEntityNames = [...selectedEntityNames, entityName];
      }
    }
  }

  function selectAll() {
    selectedEntityNames = entities.map((e) => e.name);
    selectedEndpointKeys = entities.flatMap((e) =>
      defaultOperationsForEntity(e).map((op) =>
        endpointKey(e.name, op.method, op.path),
      ),
    );
  }

  function clearSelection() {
    selectedEntityNames = [];
    selectedEndpointKeys = [];
  }

  async function chooseFrontendDirectory() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Seleccionar pasta da aplicação frontend",
      });
      if (selected && typeof selected === "string") {
        frontendPath = selected;
      }
    } catch (err) {
      errorMessage = err instanceof Error ? err.message : String(err);
    }
  }

  function methodColor(method: string): string {
    const m = method.toUpperCase();
    if (m === "GET") return "bg-emerald-500/10 text-emerald-500 border-emerald-500/20";
    if (m === "POST") return "bg-blue-500/10 text-blue-500 border-blue-500/20";
    if (m === "PUT") return "bg-amber-500/10 text-amber-500 border-amber-500/20";
    if (m === "DELETE") return "bg-rose-500/10 text-rose-500 border-rose-500/20";
    return "bg-purple-500/10 text-purple-500 border-purple-500/20";
  }

  async function handleGenerate() {
    if (selectedEntityNames.length === 0) {
      errorMessage = "Seleccione pelo menos uma entidade/endpoint para gerar o Frontend.";
      return;
    }
    if (!frontendPath.trim()) {
      errorMessage = "Indique o caminho do projecto Frontend.";
      return;
    }

    generating = true;
    errorMessage = "";
    result = null;

    try {
      const activeEntities = entities.filter((e) =>
        selectedEntityNames.includes(e.name),
      );

      const selectedEndpoints: OpenApiEndpointSelection[] = [];
      for (const entity of activeEntities) {
        const ops = defaultOperationsForEntity(entity);
        const chosenOps = ops.filter((op) =>
          selectedEndpointKeys.includes(endpointKey(entity.name, op.method, op.path)),
        );
        const finalOps = chosenOps.length > 0 ? chosenOps : ops;
        for (const op of finalOps) {
          const info = classifyOpenApiOperation(op);
          selectedEndpoints.push({
            entityName: entity.name,
            method: op.method,
            path: op.path,
            operationId: op.operationId ?? null,
            summary: op.summary ?? null,
            kind: entity.kind,
            crudRole: info.crudRole,
            patternStyle: info.patternStyle,
          });
        }
      }

      const res = await generateOpenApiFrontend({
        entities: activeEntities,
        selectedEndpoints,
        frontendPath: frontendPath.trim(),
        frontendFramework,
        apiCallPattern,
        httpClient,
        useTanstackQuery,
        backendBaseUrl: backendBaseUrl.trim() || undefined,
        theme: bootswatchUi.theme,
      });

      result = res;
    } catch (err) {
      errorMessage = err instanceof Error ? err.message : String(err);
    } finally {
      generating = false;
    }
  }
</script>

<div
  class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 overflow-y-auto"
  role="dialog"
  aria-modal="true"
>
  <div
    class="relative flex max-h-[92vh] w-full max-w-4xl flex-col rounded-2xl border border-blender-border bg-blender-card shadow-2xl overflow-hidden"
  >
    <!-- Modal Header -->
    <header class="flex items-center justify-between border-b border-blender-border px-6 py-4">
      <div class="flex items-center gap-3">
        <span class="text-xl text-blue-500">◈</span>
        <div>
          <h2 class="text-base font-semibold text-zinc-800 dark:text-zinc-100">
            Gerar Aplicação Frontend a partir de OpenAPI
          </h2>
          <p class="text-xs text-zinc-500 dark:text-zinc-400">
            Gera DTOs, API Client, TanStack Queries e páginas AdminCrud integradas com a documentação Swagger/OpenAPI.
          </p>
        </div>
      </div>
      <button
        type="button"
        onclick={onclose}
        class="rounded-lg p-1.5 text-zinc-400 hover:bg-blender-base hover:text-zinc-200 transition"
        aria-label="Fechar diálogo"
      >
        ✕
      </button>
    </header>

    <!-- Modal Body -->
    <div class="flex-1 space-y-6 overflow-y-auto p-6 text-xs text-zinc-700 dark:text-zinc-200">
      {#if errorMessage}
        <div class="rounded-lg border border-red-500/30 bg-red-500/10 p-3 text-red-500">
          <p class="font-semibold">Erro:</p>
          <p>{errorMessage}</p>
        </div>
      {/if}

      {#if result}
        <div class="rounded-xl border border-emerald-500/30 bg-emerald-500/10 p-5 space-y-3">
          <div class="flex items-center gap-2 text-emerald-500 font-semibold text-sm">
            <span>✓</span>
            <span>Frontend OpenAPI gerado com sucesso!</span>
          </div>
          <div class="grid grid-cols-3 gap-3 text-center">
            <div class="rounded-lg bg-blender-base/60 p-2.5 border border-emerald-500/20">
              <span class="block text-lg font-bold text-emerald-400">{result.endpointsProcessed}</span>
              <span class="text-[10px] text-zinc-400 uppercase tracking-wider">Entidades / Endpoints</span>
            </div>
            <div class="rounded-lg bg-blender-base/60 p-2.5 border border-emerald-500/20">
              <span class="block text-lg font-bold text-emerald-400">{result.generatedFrontendFiles}</span>
              <span class="text-[10px] text-zinc-400 uppercase tracking-wider">Ficheiros Gerados</span>
            </div>
            <div class="rounded-lg bg-blender-base/60 p-2.5 border border-emerald-500/20">
              <span class="block text-lg font-bold text-emerald-400">{result.copiedUiFiles}</span>
              <span class="text-[10px] text-zinc-400 uppercase tracking-wider">Componentes UI</span>
            </div>
          </div>
          <ul class="space-y-1 text-[11px] text-zinc-400 list-disc list-inside pt-2 border-t border-emerald-500/20">
            {#each result.summary as line}
              <li>{line}</li>
            {/each}
          </ul>
        </div>
      {/if}

      <!-- 1. Endpoint / Entity Selection -->
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
              onclick={selectAll}
              class="rounded border border-blender-border px-2 py-1 text-[10px] font-medium text-blue-400 hover:bg-blender-base transition"
            >
              Seleccionar todas
            </button>
            <button
              type="button"
              onclick={clearSelection}
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
            {@const isSelected = selectedEntityNames.includes(entity.name)}
            {@const ops = defaultOperationsForEntity(entity)}
            {@const crudInfo = summarizeEntityCrudPattern(entity)}
            <div class="p-3 hover:bg-blender-base/70 transition space-y-2">
              <div class="flex items-center justify-between gap-2">
                <label class="flex items-center gap-2.5 min-w-0 cursor-pointer flex-1">
                  <input
                    type="checkbox"
                    checked={isSelected}
                    class="rounded border-blender-border text-blue-500 focus:ring-0"
                    onchange={() => toggleEntity(entity.name)}
                  />
                  <span class="font-mono font-semibold text-zinc-200 truncate">{entity.name}</span>
                  <span class="rounded px-1.5 py-0.5 text-[9px] font-semibold uppercase bg-zinc-800 text-zinc-400 border border-zinc-700">
                    {entity.kind}
                  </span>
                  <span class="rounded border border-emerald-500/30 bg-emerald-500/10 px-1.5 py-0.5 text-[9px] text-emerald-300">
                    {crudInfo.styleLabel}
                  </span>
                </label>
                <span class="shrink-0 text-[10px] text-zinc-400">
                  {entity.fields.length} campos · {ops.length} endpoint(s)
                </span>
              </div>

              <div class="grid gap-1.5 pl-6 sm:grid-cols-2">
                {#each ops as op (`${op.method}-${op.path}`)}
                  {@const key = endpointKey(entity.name, op.method, op.path)}
                  {@const opChecked = selectedEndpointKeys.includes(key)}
                  {@const classified = classifyOpenApiOperation(op)}
                  <label
                    class={`flex items-center justify-between gap-2 rounded border px-2 py-1.5 text-[10px] cursor-pointer transition ${
                      opChecked
                        ? "border-blue-500/40 bg-blue-500/10 text-zinc-200"
                        : "border-blender-border/60 bg-blender-card/50 text-zinc-500"
                    }`}
                  >
                    <div class="flex items-center gap-1.5 min-w-0">
                      <input
                        type="checkbox"
                        checked={opChecked}
                        onchange={() => toggleEndpoint(entity.name, op)}
                        class="size-3.5 rounded border-blender-border text-blue-500"
                      />
                      <span class="rounded border px-1 py-0.2 text-[9px] font-mono font-bold {methodColor(op.method)}">
                        {op.method}
                      </span>
                      <span class="truncate font-mono" title={op.path}>{op.path}</span>
                    </div>
                    <span class="shrink-0 rounded bg-black/30 px-1.5 py-0.5 text-[8px] font-semibold uppercase tracking-wider text-zinc-300">
                      {classified.crudRole} · {classified.patternStyle === "named" ? "método" : "rest"}
                    </span>
                  </label>
                {/each}
              </div>
            </div>
          {/each}
        </div>
      </section>

      <!-- 2. Framework Frontend -->
      <section class="space-y-3">
        <h3 class="font-semibold text-zinc-800 dark:text-zinc-100">
          2. Framework Frontend de Destino
        </h3>
        <div class="grid grid-cols-3 gap-3">
          <label
            class="flex flex-col gap-1 rounded-xl border p-3 cursor-pointer transition {frontendFramework ===
            'sveltekit'
              ? 'border-blue-500/60 bg-blue-500/10 text-blue-400'
              : 'border-blender-border bg-blender-base hover:border-zinc-600'}"
          >
            <div class="flex items-center justify-between">
              <span class="font-semibold text-xs text-zinc-200">SvelteKit</span>
              <input
                type="radio"
                name="frontendFramework"
                value="sveltekit"
                bind:group={frontendFramework}
                class="text-blue-500"
              />
            </div>
            <span class="text-[10px] text-zinc-400">
              Svelte 5 Runes ($state, $derived), rotas de admin em src/routes/(admin)
            </span>
          </label>

          <label
            class="flex flex-col gap-1 rounded-xl border p-3 cursor-pointer transition {frontendFramework ===
            'nuxt'
              ? 'border-blue-500/60 bg-blue-500/10 text-blue-400'
              : 'border-blender-border bg-blender-base hover:border-zinc-600'}"
          >
            <div class="flex items-center justify-between">
              <span class="font-semibold text-xs text-zinc-200">Nuxt (Vue 3)</span>
              <input
                type="radio"
                name="frontendFramework"
                value="nuxt"
                bind:group={frontendFramework}
                class="text-blue-500"
              />
            </div>
            <span class="text-[10px] text-zinc-400">
              Vue 3 script setup, useFetch nativo / composables em pages/admin
            </span>
          </label>

          <label
            class="flex flex-col gap-1 rounded-xl border p-3 cursor-pointer transition {frontendFramework ===
            'nextjs'
              ? 'border-blue-500/60 bg-blue-500/10 text-blue-400'
              : 'border-blender-border bg-blender-base hover:border-zinc-600'}"
          >
            <div class="flex items-center justify-between">
              <span class="font-semibold text-xs text-zinc-200">Next.js (React 19)</span>
              <input
                type="radio"
                name="frontendFramework"
                value="nextjs"
                bind:group={frontendFramework}
                class="text-blue-500"
              />
            </div>
            <span class="text-[10px] text-zinc-400">
              App Router, componentes "use client" em app/admin/[entity]
            </span>
          </label>
        </div>
      </section>

      <!-- 3. Frontend Destination Path -->
      <section class="space-y-2">
        <label for="frontendPath" class="block font-semibold text-zinc-800 dark:text-zinc-100">
          3. Pasta do Projecto Frontend
        </label>
        <div class="flex gap-2">
          <input
            id="frontendPath"
            type="text"
            bind:value={frontendPath}
            placeholder="/caminho/para/projecto-frontend"
            class="flex-1 rounded-lg border border-blender-border bg-blender-base px-3 py-2 text-xs text-zinc-200 placeholder-zinc-500 focus:border-blue-500 focus:outline-none"
          />
          <button
            type="button"
            onclick={chooseFrontendDirectory}
            class="rounded-lg border border-blender-border bg-blender-base px-3 py-2 text-xs font-medium text-zinc-300 hover:bg-blender-base/80 transition"
          >
            Procurar...
          </button>
        </div>
      </section>

      <!-- 4. Communication Pattern (BFF vs Direct) -->
      <section class="space-y-3">
        <h3 class="font-semibold text-zinc-800 dark:text-zinc-100">
          4. Padrão de Comunicação (BFF vs Direct)
        </h3>
        <div class="grid grid-cols-2 gap-3">
          <label
            class="flex flex-col gap-1 rounded-xl border p-3 cursor-pointer transition {apiCallPattern ===
            'bff'
              ? 'border-blue-500/60 bg-blue-500/10 text-blue-400'
              : 'border-blender-border bg-blender-base hover:border-zinc-600'}"
          >
            <div class="flex items-center justify-between">
              <span class="font-semibold text-xs text-zinc-200">BFF (Backend-for-Frontend)</span>
              <input
                type="radio"
                name="apiCallPattern"
                value="bff"
                bind:group={apiCallPattern}
                class="text-blue-500"
              />
            </div>
            <span class="text-[10px] text-zinc-400">
              Cria rotas de proxy no próprio frontend (+server.ts, server/api, route.ts) protegendo chaves e centralizando chamadas
            </span>
          </label>

          <label
            class="flex flex-col gap-1 rounded-xl border p-3 cursor-pointer transition {apiCallPattern ===
            'direct'
              ? 'border-blue-500/60 bg-blue-500/10 text-blue-400'
              : 'border-blender-border bg-blender-base hover:border-zinc-600'}"
          >
            <div class="flex items-center justify-between">
              <span class="font-semibold text-xs text-zinc-200">Chamada Direta ao Backend</span>
              <input
                type="radio"
                name="apiCallPattern"
                value="direct"
                bind:group={apiCallPattern}
                class="text-blue-500"
              />
            </div>
            <span class="text-[10px] text-zinc-400">
              O cliente no browser chama a URL base do backend de forma direta via CORS
            </span>
          </label>
        </div>
      </section>

      <!-- 5. Backend Base URL -->
      <section class="space-y-2">
        <label for="backendBaseUrl" class="block font-semibold text-zinc-800 dark:text-zinc-100">
          5. URL Base do Backend
        </label>
        <input
          id="backendBaseUrl"
          type="text"
          bind:value={backendBaseUrl}
          placeholder="http://localhost:8080 ou https://api.exemplo.com"
          class="w-full rounded-lg border border-blender-border bg-blender-base px-3 py-2 text-xs text-zinc-200 placeholder-zinc-500 focus:border-blue-500 focus:outline-none"
        />
      </section>

      <!-- 6. HTTP Client (Native vs Axios) -->
      <section class="space-y-3">
        <h3 class="font-semibold text-zinc-800 dark:text-zinc-100">
          6. Cliente de Requisição HTTP
        </h3>
        <div class="grid grid-cols-2 gap-3">
          <label
            class="flex flex-col gap-1 rounded-xl border p-3 cursor-pointer transition {httpClient ===
            'native'
              ? 'border-blue-500/60 bg-blue-500/10 text-blue-400'
              : 'border-blender-border bg-blender-base hover:border-zinc-600'}"
          >
            <div class="flex items-center justify-between">
              <span class="font-semibold text-xs text-zinc-200">
                Nativo do Framework ({frontendFramework === "nuxt"
                  ? "useFetch / $fetch"
                  : "Fetch API nativa"})
              </span>
              <input
                type="radio"
                name="httpClient"
                value="native"
                bind:group={httpClient}
                class="text-blue-500"
              />
            </div>
            <span class="text-[10px] text-zinc-400">
              {frontendFramework === "nuxt"
                ? "Utiliza o useFetch() e $fetch() nativos do ecossistema Nuxt / Nitro"
                : "Utiliza a Fetch API padrão sem dependências externas adicionais"}
            </span>
          </label>

          <label
            class="flex flex-col gap-1 rounded-xl border p-3 cursor-pointer transition {httpClient ===
            'axios'
              ? 'border-blue-500/60 bg-blue-500/10 text-blue-400'
              : 'border-blender-border bg-blender-base hover:border-zinc-600'}"
          >
            <div class="flex items-center justify-between">
              <span class="font-semibold text-xs text-zinc-200">Axios</span>
              <input
                type="radio"
                name="httpClient"
                value="axios"
                bind:group={httpClient}
                class="text-blue-500"
              />
            </div>
            <span class="text-[10px] text-zinc-400">
              Instância configurada com axios.create(), interceptors e métodos tipados
            </span>
          </label>
        </div>
      </section>

      <!-- 7. TanStack Query -->
      <section class="space-y-2">
        <div class="flex items-center justify-between rounded-xl border border-blender-border bg-blender-base p-3">
          <div>
            <span class="block font-semibold text-xs text-zinc-200">
              7. Integrar TanStack Query ({frontendFramework === "nuxt"
                ? "@tanstack/vue-query"
                : frontendFramework === "sveltekit"
                  ? "@tanstack/svelte-query"
                  : "@tanstack/react-query"})
            </span>
            <span class="text-[10px] text-zinc-400">
              Gera hooks e composables com cache reactivo, invalidação automática em mutations e paginação
            </span>
          </div>
          <input
            type="checkbox"
            bind:checked={useTanstackQuery}
            class="h-5 w-5 rounded border-blender-border text-blue-500 focus:ring-0"
          />
        </div>
      </section>
    </div>

    <!-- Modal Footer -->
    <footer class="flex items-center justify-end gap-3 border-t border-blender-border px-6 py-4">
      <button
        type="button"
        onclick={onclose}
        class="rounded-lg border border-blender-border px-4 py-2 text-xs font-semibold text-zinc-300 hover:bg-blender-base transition"
      >
        Fechar
      </button>
      <button
        type="button"
        onclick={handleGenerate}
        disabled={generating || selectedEntityNames.length === 0}
        class="rounded-lg bg-blue-600 px-5 py-2 text-xs font-semibold text-white shadow-sm hover:bg-blue-500 disabled:opacity-50 transition"
      >
        {generating ? "A gerar Frontend..." : "◈ Gerar Frontend (AdminCrud)"}
      </button>
    </footer>
  </div>
</div>
