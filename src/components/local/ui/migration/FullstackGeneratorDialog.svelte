<script lang="ts">
  import { LocalStyleComponent } from "../../data/style.component";
  import { activeProject } from "../../data/projects";
  import type { ApiCallPattern, ArchitectureMode, BackendFramework, DatabaseEngine, DatabaseTable, FrontendFramework, FullstackCodegenResult, HttpClientKind, SpringLanguage } from "../../data/migration";
  import { initialTableSelection, runFullstackGeneration } from "./fullstack-gen/generate";
  import TopologySection from "./fullstack-gen/TopologySection.svelte";
  import TableSelectionSection from "./fullstack-gen/TableSelectionSection.svelte";
  import FrontendSection from "./fullstack-gen/FrontendSection.svelte";
  import BackendSection from "./fullstack-gen/BackendSection.svelte";
  import DatabaseEngineSection from "./fullstack-gen/DatabaseEngineSection.svelte";
  import GenerationResult from "./fullstack-gen/GenerationResult.svelte";

  let { tables = [], initialSelectedTables = [], onclose }: { tables: DatabaseTable[]; initialSelectedTables?: string[]; onclose: () => void } = $props();

  // svelte-ignore state_referenced_locally
  let selectedTableNames = $state<string[]>(initialTableSelection(tables, initialSelectedTables));
  let architectureMode = $state<ArchitectureMode>("split");
  let frontendFramework = $state<FrontendFramework>("sveltekit");
  let frontendPath = $state($activeProject?.path ?? "");
  let apiCallPattern = $state<ApiCallPattern>("bff");
  let httpClient = $state<HttpClientKind>("native");
  let useTanstackQuery = $state<boolean>(true);
  let backendBaseUrl = $state("http://localhost:8080");
  let backendFramework = $state<BackendFramework>("spring-boot");
  let backendPath = $state("");
  let basePackage = $state("");
  let springLanguage = $state<SpringLanguage>("java");
  let ormStrategy = $state("spring-data-jpa");
  let databaseEngine = $state<DatabaseEngine>("postgres");
  let generating = $state(false);
  let errorMessage = $state("");
  let result = $state<FullstackCodegenResult | null>(null);

  async function handleGenerate(event: SubmitEvent) {
    event.preventDefault();
    errorMessage = "";
    result = null;
    generating = true;
    try {
      const outcome = await runFullstackGeneration({ architectureMode, tables, selectedTableNames, frontendPath, frontendFramework, apiCallPattern, httpClient, useTanstackQuery, backendPath, backendFramework, basePackage, springLanguage, ormStrategy, databaseEngine, backendBaseUrl });
      result = outcome.result;
      errorMessage = outcome.errorMessage;
    } finally {
      generating = false;
    }
  }
</script>

<div class={LocalStyleComponent.migration.overlay} role="presentation" onclick={(e) => e.target === e.currentTarget && !generating && onclose()}>
  <div class="flex max-h-[92vh] w-full max-w-5xl flex-col overflow-hidden rounded-2xl border border-blender-border bg-blender-card text-zinc-100 shadow-2xl" role="dialog" aria-modal="true" aria-labelledby="fullstack-gen-title">
    <header class="flex items-center justify-between border-b border-blender-border px-6 py-4">
      <div>
        <span class="text-[10px] font-semibold uppercase tracking-[0.18em] text-blue-400">Gerador Full-Stack · Feature-Based Architecture</span>
        <h2 id="fullstack-gen-title" class="mt-0.5 text-lg font-semibold">Gerar Backend & Frontend CRUD (`AdminCrud`)</h2>
      </div>
      <button type="button" class="rounded px-2.5 py-1 text-zinc-400 hover:bg-blender-hover hover:text-zinc-100" onclick={onclose} disabled={generating}>✕</button>
    </header>

    <form onsubmit={handleGenerate} class="flex min-h-0 flex-1 flex-col">
      <div class="flex-1 space-y-6 overflow-y-auto p-6">
        <TopologySection bind:architectureMode />
        <TableSelectionSection {tables} bind:selectedTableNames />
        <FrontendSection {architectureMode} bind:frontendFramework bind:frontendPath bind:apiCallPattern bind:httpClient bind:useTanstackQuery />
        <!-- 4. Configuração do Backend (Split vs Monolith) -->
        {#if architectureMode === "split"}
          <BackendSection bind:backendFramework bind:backendPath bind:basePackage bind:springLanguage bind:backendBaseUrl bind:ormStrategy bind:databaseEngine />
        {:else}
          <DatabaseEngineSection {frontendFramework} bind:databaseEngine />
        {/if}
        <GenerationResult {errorMessage} {result} />
      </div>

      <footer class="flex items-center justify-between border-t border-blender-border bg-blender-card px-6 py-4">
        <span class="text-xs text-zinc-400">Métodos gerados por tabela: <code class="text-zinc-200">findAll</code>, <code class="text-zinc-200">insert</code>, <code class="text-zinc-200">update</code>, <code class="text-zinc-200">delete</code></span>
        <div class="flex gap-3">
          <button type="button" class={LocalStyleComponent.migration.ghostButton} onclick={onclose} disabled={generating}>Fechar</button>
          <button type="submit" class={LocalStyleComponent.migration.primaryButton} disabled={generating || selectedTableNames.length === 0}>{generating ? "A gerar projecto…" : "Gerar Aplicação CRUD"}</button>
        </div>
      </footer>
    </form>
  </div>
</div>
