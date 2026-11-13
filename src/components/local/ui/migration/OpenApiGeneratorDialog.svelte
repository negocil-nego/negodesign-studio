<script lang="ts">
  import { activeProject } from "../../data/projects";
  import type { ApiCallPattern, FrontendFramework, HttpClientKind, OpenApiEntity, OpenApiFrontendCodegenResult, OpenApiOperation } from "../../data/migration";
  import { filterEntities } from "./openapi-gen/endpoints";
  import { runOpenApiGeneration } from "./openapi-gen/generate";
  import { emptySelection, initialSelection, selectAllEntities, toggleEndpointSelection, toggleEntitySelection, type Selection } from "./openapi-gen/selection";
  import ConnectionOptions from "./openapi-gen/ConnectionOptions.svelte";
  import EndpointPicker from "./openapi-gen/EndpointPicker.svelte";
  import FrontendFrameworkPicker from "./openapi-gen/FrontendFrameworkPicker.svelte";
  import GenerationResult from "./openapi-gen/GenerationResult.svelte";
  import HttpTanstackOptions from "./openapi-gen/HttpTanstackOptions.svelte";

  let {
    entities = [],
    initialSelectedEntityNames = [],
    onclose,
  }: { entities: OpenApiEntity[]; initialSelectedEntityNames?: string[]; onclose: () => void } = $props();

  // svelte-ignore state_referenced_locally
  let sel = $state<Selection>(initialSelection(entities, initialSelectedEntityNames));
  const selectedEntityNames = $derived(sel.entityNames);
  const selectedEndpointKeys = $derived(sel.endpointKeys);
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

  const filteredEntities = $derived(filterEntities(entities, searchQuery));

  const toggleEntity = (name: string) => (sel = toggleEntitySelection(entities, sel, name));
  const toggleEndpoint = (entityName: string, op: OpenApiOperation) =>
    (sel = toggleEndpointSelection(entities, sel, entityName, op));
  const selectAll = () => (sel = selectAllEntities(entities));
  const clearSelection = () => (sel = emptySelection());

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
      const res = await runOpenApiGeneration({
        entities, selectedEntityNames, selectedEndpointKeys, frontendPath, frontendFramework,
        apiCallPattern, httpClient, useTanstackQuery, backendBaseUrl,
      });
      result = res.result;
      errorMessage = res.errorMessage;
    } finally {
      generating = false;
    }
  }
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 overflow-y-auto" role="dialog" aria-modal="true">
  <div class="relative flex max-h-[92vh] w-full max-w-4xl flex-col rounded-2xl border border-blender-border bg-blender-card shadow-2xl overflow-hidden">
    <header class="flex items-center justify-between border-b border-blender-border px-6 py-4">
      <div class="flex items-center gap-3">
        <span class="text-xl text-blue-500">◈</span>
        <div>
          <h2 class="text-base font-semibold text-zinc-800 dark:text-zinc-100">Gerar Aplicação Frontend a partir de OpenAPI</h2>
          <p class="text-xs text-zinc-500 dark:text-zinc-400">Gera DTOs, API Client, TanStack Queries e páginas AdminCrud integradas com a documentação Swagger/OpenAPI.</p>
        </div>
      </div>
      <button type="button" onclick={onclose} class="rounded-lg p-1.5 text-zinc-400 hover:bg-blender-base hover:text-zinc-200 transition" aria-label="Fechar diálogo">✕</button>
    </header>

    <div class="flex-1 space-y-6 overflow-y-auto p-6 text-xs text-zinc-700 dark:text-zinc-200">
      <GenerationResult {errorMessage} {result} />
      <EndpointPicker {entities} {filteredEntities} {selectedEntityNames} {selectedEndpointKeys} bind:searchQuery
        ontoggleentity={toggleEntity} ontoggleendpoint={toggleEndpoint} onselectall={selectAll} onclear={clearSelection} />
      <FrontendFrameworkPicker bind:frontendFramework />
      <ConnectionOptions bind:frontendPath bind:apiCallPattern bind:backendBaseUrl onerror={(m) => (errorMessage = m)} />
      <HttpTanstackOptions {frontendFramework} bind:httpClient bind:useTanstackQuery />
    </div>

    <footer class="flex items-center justify-end gap-3 border-t border-blender-border px-6 py-4">
      <button type="button" onclick={onclose} class="rounded-lg border border-blender-border px-4 py-2 text-xs font-semibold text-zinc-300 hover:bg-blender-base transition">Fechar</button>
      <button type="button" onclick={handleGenerate} disabled={generating || selectedEntityNames.length === 0} class="rounded-lg bg-blue-600 px-5 py-2 text-xs font-semibold text-white shadow-sm hover:bg-blue-500 disabled:opacity-50 transition">
        {generating ? "A gerar Frontend..." : "◈ Gerar Frontend (AdminCrud)"}
      </button>
    </footer>
  </div>
</div>
