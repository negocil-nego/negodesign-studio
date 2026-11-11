<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { LocalStyleComponent } from "../../data/style.component";
  import { activeProject } from "../../data/projects";
  import { bootswatchUi } from "../../data/bootswatch-ui.svelte";
  import {
    generateFullstackCrud,
    type ApiCallPattern,
    type ArchitectureMode,
    type BackendFramework,
    type DatabaseEngine,
    type DatabaseTable,
    type FrontendFramework,
    type FullstackCodegenResult,
    type HttpClientKind,
  } from "../../data/migration";

  let {
    tables = [],
    initialSelectedTables = [],
    onclose,
  }: {
    tables: DatabaseTable[];
    initialSelectedTables?: string[];
    onclose: () => void;
  } = $props();

  // svelte-ignore state_referenced_locally
  let selectedTableNames = $state<string[]>(
    initialSelectedTables.length > 0
      ? [...initialSelectedTables]
      : tables.map((t) => t.name),
  );

  let architectureMode = $state<ArchitectureMode>("split");
  let frontendFramework = $state<FrontendFramework>("sveltekit");
  let frontendPath = $state($activeProject?.path ?? "");
  let apiCallPattern = $state<ApiCallPattern>("bff");
  let httpClient = $state<HttpClientKind>("native");
  let useTanstackQuery = $state<boolean>(true);
  let backendBaseUrl = $state("http://localhost:8080");

  let backendFramework = $state<BackendFramework>("spring-boot");
  let backendPath = $state("");
  let basePackage = $state("com.negocil.negoturismo.admin");
  let ormStrategy = $state("spring-data-jpa");
  let databaseEngine = $state<DatabaseEngine>("postgres");

  let generating = $state(false);
  let errorMessage = $state("");
  let result = $state<FullstackCodegenResult | null>(null);

  const ormOptionsByBackend: Record<
    BackendFramework,
    { id: string; label: string; desc: string }[]
  > = {
    "spring-boot": [
      {
        id: "spring-data-jpa",
        label: "Spring Data JPA",
        desc: "Entidades Jakarta Persistence + JpaRepository paginado",
      },
      {
        id: "spring-jdbc",
        label: "Spring JDBC (Nova API JdbcClient)",
        desc: "Consultas fluentes com org.springframework.jdbc.core.simple.JdbcClient",
      },
    ],
    nestjs: [
      {
        id: "prisma",
        label: "Prisma ORM",
        desc: "Repositórios tipados usando PrismaClient (findMany, create, update, delete)",
      },
      {
        id: "drizzle",
        label: "Drizzle ORM",
        desc: "Consultas SQL-like tipadas com Drizzle",
      },
      {
        id: "typeorm",
        label: "TypeORM",
        desc: "Entidades decoradas e Repository<T> do TypeORM",
      },
    ],
    laravel: [
      {
        id: "eloquent",
        label: "Eloquent ORM",
        desc: "Models Eloquent activos para CRUD e paginação",
      },
      {
        id: "query-builder",
        label: "Query Builder",
        desc: "Consultas fluentes com Illuminate\\Support\\Facades\\DB::table()",
      },
      {
        id: "raw-sql",
        label: "Raw SQL",
        desc: "Consultas nativas parametrizadas (DB::select, DB::insert, DB::update, DB::delete)",
      },
    ],
    "aspnet-core": [
      {
        id: "ef-core",
        label: "Entity Framework Core",
        desc: "DbContext e DbSet<T> assíncronos",
      },
      {
        id: "dapper",
        label: "Dapper",
        desc: "Micro-ORM de alta performance sobre IDbConnection",
      },
      {
        id: "raw-sql",
        label: "Raw SQL (ADO.NET)",
        desc: "DbConnection e DbCommand parametrizados",
      },
    ],
  };

  const currentOrmOptions = $derived(ormOptionsByBackend[backendFramework]);

  const tanstackPackageLabel = $derived(
    frontendFramework === "sveltekit"
      ? "@tanstack/svelte-query"
      : frontendFramework === "nuxt"
        ? "@tanstack/vue-query"
        : "@tanstack/react-query",
  );

  const nativeHttpClientLabel = $derived(
    frontendFramework === "nuxt"
      ? "useFetch / $fetch (Nativo Nuxt)"
      : "Fetch API (Nativo Web / SSR)",
  );

  function selectBackendFramework(fw: BackendFramework) {
    backendFramework = fw;
    ormStrategy = ormOptionsByBackend[fw][0].id;
  }

  function toggleTable(name: string) {
    if (selectedTableNames.includes(name)) {
      selectedTableNames = selectedTableNames.filter((n) => n !== name);
    } else {
      selectedTableNames = [...selectedTableNames, name];
    }
  }

  function selectAllTables() {
    selectedTableNames = tables.map((t) => t.name);
  }

  function clearSelectedTables() {
    selectedTableNames = [];
  }

  async function pickFolder(target: "frontend" | "backend") {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title:
          target === "frontend"
            ? "Seleccionar pasta do projecto Frontend"
            : "Seleccionar pasta do projecto Backend",
      });
      if (typeof selected === "string" && selected.trim()) {
        if (target === "frontend") frontendPath = selected;
        else backendPath = selected;
      }
    } catch (err) {
      console.error("Erro ao abrir selector de pastas:", err);
    }
  }

  async function handleGenerate(event: SubmitEvent) {
    event.preventDefault();
    errorMessage = "";
    result = null;

    const chosenTables = tables.filter((t) =>
      selectedTableNames.includes(t.name),
    );
    if (chosenTables.length === 0) {
      errorMessage = "Seleccione pelo menos uma tabela para gerar.";
      return;
    }

    generating = true;
    try {
      result = await generateFullstackCrud({
        architectureMode,
        tables: chosenTables,
        frontendPath,
        frontendFramework,
        apiCallPattern,
        httpClient,
        useTanstackQuery,
        backendPath: architectureMode === "split" ? backendPath : undefined,
        backendFramework:
          architectureMode === "split" ? backendFramework : undefined,
        ormStrategy: architectureMode === "split" ? ormStrategy : undefined,
        basePackage:
          architectureMode === "split" && backendFramework === "spring-boot"
            ? basePackage
            : undefined,
        databaseEngine,
        backendBaseUrl,
        theme: bootswatchUi.theme,
      });
    } catch (err) {
      errorMessage = err instanceof Error ? err.message : String(err);
    } finally {
      generating = false;
    }
  }
</script>

<div
  class={LocalStyleComponent.migration.overlay}
  role="presentation"
  onclick={(e) => e.target === e.currentTarget && !generating && onclose()}
>
  <div
    class="flex max-h-[92vh] w-full max-w-5xl flex-col overflow-hidden rounded-2xl border border-blender-border bg-blender-card text-zinc-100 shadow-2xl"
    role="dialog"
    aria-modal="true"
    aria-labelledby="fullstack-gen-title"
  >
    <header
      class="flex items-center justify-between border-b border-blender-border px-6 py-4"
    >
      <div>
        <span
          class="text-[10px] font-semibold uppercase tracking-[0.18em] text-blue-400"
        >
          Gerador Full-Stack · Feature-Based Architecture
        </span>
        <h2 id="fullstack-gen-title" class="mt-0.5 text-lg font-semibold">
          Gerar Backend & Frontend CRUD (`AdminCrud`)
        </h2>
      </div>
      <button
        type="button"
        class="rounded px-2.5 py-1 text-zinc-400 hover:bg-blender-hover hover:text-zinc-100"
        onclick={onclose}
        disabled={generating}
      >
        ✕
      </button>
    </header>

    <form onsubmit={handleGenerate} class="flex min-h-0 flex-1 flex-col">
      <div class="flex-1 space-y-6 overflow-y-auto p-6">
        <!-- 1. Topologia da Aplicação -->
        <section class="space-y-3">
          <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-400">
            1. Topologia da Aplicação
          </h3>
          <div class="grid gap-3 sm:grid-cols-2">
            <button
              type="button"
              class={`flex flex-col items-start rounded-xl border p-4 text-left transition ${
                architectureMode === "split"
                  ? "border-blue-500 bg-blue-500/10 text-zinc-100"
                  : "border-blender-border bg-blender-base text-zinc-400 hover:border-zinc-600"
              }`}
              onclick={() => (architectureMode = "split")}
            >
              <span class="text-sm font-semibold text-zinc-100">
                Dividir em Duas Aplicações (Frontend + Backend)
              </span>
              <span class="mt-1 text-xs text-zinc-400">
                Gera o backend (Spring Boot, NestJS, Laravel ou ASP.NET Core) no
                padrão `feature` (controller → service → repository) e integra
                com o projecto frontend.
              </span>
            </button>

            <button
              type="button"
              class={`flex flex-col items-start rounded-xl border p-4 text-left transition ${
                architectureMode === "monolith"
                  ? "border-blue-500 bg-blue-500/10 text-zinc-100"
                  : "border-blender-border bg-blender-base text-zinc-400 hover:border-zinc-600"
              }`}
              onclick={() => (architectureMode = "monolith")}
            >
              <span class="text-sm font-semibold text-zinc-100">
                Tudo numa Única Aplicação (Fullstack Frontend + API Direta)
              </span>
              <span class="mt-1 text-xs text-zinc-400">
                Utiliza o próprio SvelteKit, Nuxt ou Next.js como backend criando
                rotas de API com ligação direta a PostgreSQL, MySQL ou SQLite.
              </span>
            </button>
          </div>
        </section>

        <!-- 2. Selecção de Tabelas -->
        <section class="space-y-3 rounded-xl border border-blender-border bg-blender-base/50 p-4">
          <div class="flex flex-wrap items-center justify-between gap-2">
            <div>
              <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-300">
                2. Tabelas a Gerar ({selectedTableNames.length} de {tables.length})
              </h3>
              <p class="text-[11px] text-zinc-500">
                Escolha todas as tabelas ou apenas algumas para gerar os endpoints e o menu lateral do `AdminCrud`.
              </p>
            </div>
            <div class="flex gap-2">
              <button
                type="button"
                class="rounded border border-blender-border bg-blender-card px-2.5 py-1 text-[11px] text-zinc-300 hover:bg-blender-hover"
                onclick={selectAllTables}
              >
                Seleccionar todas
              </button>
              <button
                type="button"
                class="rounded border border-blender-border bg-blender-card px-2.5 py-1 text-[11px] text-zinc-400 hover:bg-blender-hover"
                onclick={clearSelectedTables}
              >
                Limpar
              </button>
            </div>
          </div>

          <div class="grid max-h-36 grid-cols-2 gap-2 overflow-y-auto sm:grid-cols-3 md:grid-cols-4">
            {#each tables as tbl (tbl.name)}
              {@const checked = selectedTableNames.includes(tbl.name)}
              <label
                class={`flex cursor-pointer items-center gap-2 rounded-lg border px-3 py-2 text-xs transition ${
                  checked
                    ? "border-blue-500/60 bg-blue-500/10 text-zinc-100"
                    : "border-blender-border bg-blender-card text-zinc-400"
                }`}
              >
                <input
                  type="checkbox"
                  checked={checked}
                  onchange={() => toggleTable(tbl.name)}
                  class="rounded border-blender-border text-blue-600"
                />
                <span class="truncate font-mono" title={tbl.name}>{tbl.name}</span>
              </label>
            {/each}
          </div>
        </section>

        <!-- 3. Aplicação Frontend -->
        <section class="space-y-4 rounded-xl border border-blender-border bg-blender-base/50 p-4">
          <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-300">
            3. Configuração do Projecto Frontend (`AdminCrud`)
          </h3>

          <div class="grid gap-4 sm:grid-cols-3">
            {#each [
              { id: "sveltekit", label: "SvelteKit", desc: "Rotas +page.svelte & AdminCrud.svelte" },
              { id: "nuxt", label: "Nuxt", desc: "Páginas Vue 3, useFetch & AdminCrud.vue" },
              { id: "nextjs", label: "Next.js", desc: "App Router React & AdminCrud.tsx" },
            ] as fw (fw.id)}
              <button
                type="button"
                class={`rounded-lg border p-3 text-left transition ${
                  frontendFramework === fw.id
                    ? "border-blue-500 bg-blue-500/15 text-zinc-100"
                    : "border-blender-border bg-blender-card text-zinc-400 hover:border-zinc-600"
                }`}
                onclick={() => (frontendFramework = fw.id as FrontendFramework)}
              >
                <span class="block text-xs font-semibold text-zinc-100">{fw.label}</span>
                <span class="mt-0.5 block text-[11px] text-zinc-400">{fw.desc}</span>
              </button>
            {/each}
          </div>

          <div class="grid gap-4 sm:grid-cols-2">
            <label class="block sm:col-span-2">
              <span class={LocalStyleComponent.migration.label}>
                Caminho da Aplicação Frontend
              </span>
              <div class="mt-1 flex gap-2">
                <input
                  class={LocalStyleComponent.migration.field}
                  bind:value={frontendPath}
                  placeholder="/caminho/para/projecto-frontend"
                  required
                />
                <button
                  type="button"
                  class="shrink-0 rounded-lg border border-blender-border bg-blender-card px-3 py-2 text-xs font-medium text-zinc-200 hover:bg-blender-hover"
                  onclick={() => pickFolder("frontend")}
                >
                  Procurar…
                </button>
              </div>
            </label>

            {#if architectureMode === "split"}
              <div>
                <span class={LocalStyleComponent.migration.label}>
                  Padrão de Comunicação com o Backend
                </span>
                <div class="mt-1 grid grid-cols-2 gap-2">
                  <button
                    type="button"
                    class={`rounded-lg border px-3 py-2 text-xs font-medium transition ${
                      apiCallPattern === "bff"
                        ? "border-blue-500 bg-blue-500/15 text-blue-300"
                        : "border-blender-border bg-blender-card text-zinc-400"
                    }`}
                    onclick={() => (apiCallPattern = "bff")}
                  >
                    Padrão BFF (Server Routes)
                  </button>
                  <button
                    type="button"
                    class={`rounded-lg border px-3 py-2 text-xs font-medium transition ${
                      apiCallPattern === "direct"
                        ? "border-blue-500 bg-blue-500/15 text-blue-300"
                        : "border-blender-border bg-blender-card text-zinc-400"
                    }`}
                    onclick={() => (apiCallPattern = "direct")}
                  >
                    Chamar Backend Diretamente
                  </button>
                </div>
              </div>
            {/if}

            <div>
              <span class={LocalStyleComponent.migration.label}>
                Cliente HTTP de Requisição
              </span>
              <div class="mt-1 grid grid-cols-2 gap-2">
                <button
                  type="button"
                  class={`rounded-lg border px-3 py-2 text-xs font-medium transition ${
                    httpClient === "native"
                      ? "border-blue-500 bg-blue-500/15 text-blue-300"
                      : "border-blender-border bg-blender-card text-zinc-400"
                  }`}
                  onclick={() => (httpClient = "native")}
                >
                  {nativeHttpClientLabel}
                </button>
                <button
                  type="button"
                  class={`rounded-lg border px-3 py-2 text-xs font-medium transition ${
                    httpClient === "axios"
                      ? "border-blue-500 bg-blue-500/15 text-blue-300"
                      : "border-blender-border bg-blender-card text-zinc-400"
                  }`}
                  onclick={() => (httpClient = "axios")}
                >
                  Axios
                </button>
              </div>
            </div>

            <div class="flex items-center justify-between rounded-lg border border-blender-border bg-blender-card px-4 py-2.5 sm:col-span-2">
              <div>
                <span class="block text-xs font-semibold text-zinc-200">
                  Integrar TanStack Query ({tanstackPackageLabel})
                </span>
                <span class="block text-[11px] text-zinc-400">
                  Gera hooks reactivos de query e mutação (findAll, insert, update, delete) com invalidação de cache.
                </span>
              </div>
              <input
                type="checkbox"
                bind:checked={useTanstackQuery}
                class="size-4 rounded border-blender-border text-blue-600"
              />
            </div>
          </div>
        </section>

        <!-- 4. Configuração do Backend (Split vs Monolith) -->
        {#if architectureMode === "split"}
          <section class="space-y-4 rounded-xl border border-blender-border bg-blender-base/50 p-4">
            <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-300">
              4. Configuração do Backend & Estratégia de Repositório (ORM)
            </h3>

            <div class="grid gap-3 sm:grid-cols-4">
              {#each [
                { id: "spring-boot", label: "Spring Boot", sub: "Java · Modulith Feature" },
                { id: "nestjs", label: "NestJS", sub: "TypeScript · Feature Modules" },
                { id: "laravel", label: "Laravel", sub: "PHP · Feature Architecture" },
                { id: "aspnet-core", label: "ASP.NET Core", sub: "C# · Feature Controllers" },
              ] as bfw (bfw.id)}
                <button
                  type="button"
                  class={`rounded-lg border p-3 text-left transition ${
                    backendFramework === bfw.id
                      ? "border-blue-500 bg-blue-500/15 text-zinc-100"
                      : "border-blender-border bg-blender-card text-zinc-400 hover:border-zinc-600"
                  }`}
                  onclick={() => selectBackendFramework(bfw.id as BackendFramework)}
                >
                  <span class="block text-xs font-semibold text-zinc-100">{bfw.label}</span>
                  <span class="mt-0.5 block text-[10px] text-zinc-400">{bfw.sub}</span>
                </button>
              {/each}
            </div>

            <div class="grid gap-4 sm:grid-cols-2">
              <label class="block sm:col-span-2">
                <span class={LocalStyleComponent.migration.label}>
                  Caminho da Aplicação Backend
                </span>
                <div class="mt-1 flex gap-2">
                  <input
                    class={LocalStyleComponent.migration.field}
                    bind:value={backendPath}
                    placeholder="/caminho/para/projecto-backend"
                    required
                  />
                  <button
                    type="button"
                    class="shrink-0 rounded-lg border border-blender-border bg-blender-card px-3 py-2 text-xs font-medium text-zinc-200 hover:bg-blender-hover"
                    onclick={() => pickFolder("backend")}
                  >
                    Procurar…
                  </button>
                </div>
              </label>

              {#if backendFramework === "spring-boot"}
                <label class="block">
                  <span class={LocalStyleComponent.migration.label}>
                    Pacote Base Java (Feature Package)
                  </span>
                  <input
                    class={LocalStyleComponent.migration.field}
                    bind:value={basePackage}
                    placeholder="com.negocil.negoturismo.admin"
                  />
                </label>
              {/if}

              <label class="block">
                <span class={LocalStyleComponent.migration.label}>
                  URL Base do Servidor Backend
                </span>
                <input
                  class={LocalStyleComponent.migration.field}
                  bind:value={backendBaseUrl}
                  placeholder="http://localhost:8080"
                />
              </label>
            </div>

            <div class="space-y-2">
              <span class={LocalStyleComponent.migration.label}>
                Estratégia do Repositório / ORM para {backendFramework}
              </span>
              <div class="grid gap-3 sm:grid-cols-3">
                {#each currentOrmOptions as opt (opt.id)}
                  <button
                    type="button"
                    class={`rounded-lg border p-3 text-left transition ${
                      ormStrategy === opt.id
                        ? "border-emerald-500 bg-emerald-500/10 text-zinc-100"
                        : "border-blender-border bg-blender-card text-zinc-400 hover:border-zinc-600"
                    }`}
                    onclick={() => (ormStrategy = opt.id)}
                  >
                    <span class="block text-xs font-semibold text-zinc-100">{opt.label}</span>
                    <span class="mt-1 block text-[11px] leading-relaxed text-zinc-400">{opt.desc}</span>
                  </button>
                {/each}
              </div>
            </div>
          </section>
        {:else}
          <section class="space-y-3 rounded-xl border border-blender-border bg-blender-base/50 p-4">
            <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-300">
              4. Motor de Base de Dados para Conexão Direta nas Rotas de API ({frontendFramework})
            </h3>
            <div class="grid gap-3 sm:grid-cols-3">
              {#each [
                { id: "postgres", label: "PostgreSQL", desc: "Pool de conexão direta via pg" },
                { id: "mysql", label: "MySQL", desc: "Pool de conexão direta via mysql2/promise" },
                { id: "sqlite", label: "SQLite", desc: "Conexão local direta via better-sqlite3" },
              ] as db (db.id)}
                <button
                  type="button"
                  class={`rounded-lg border p-3 text-left transition ${
                    databaseEngine === db.id
                      ? "border-emerald-500 bg-emerald-500/10 text-zinc-100"
                      : "border-blender-border bg-blender-card text-zinc-400 hover:border-zinc-600"
                  }`}
                  onclick={() => (databaseEngine = db.id as DatabaseEngine)}
                >
                  <span class="block text-xs font-semibold text-zinc-100">{db.label}</span>
                  <span class="mt-0.5 block text-[11px] text-zinc-400">{db.desc}</span>
                </button>
              {/each}
            </div>
          </section>
        {/if}

        {#if errorMessage}
          <p class={LocalStyleComponent.migration.error} role="alert">
            {errorMessage}
          </p>
        {/if}

        {#if result}
          <div class="rounded-xl border border-emerald-500/40 bg-emerald-500/10 p-4 text-xs text-emerald-200 space-y-2">
            <p class="font-semibold text-emerald-300">
              Geração concluída com sucesso ({result.tablesProcessed} tabela(s) processada(s))!
            </p>
            <ul class="list-disc pl-5 space-y-1">
              {#each result.summary as line}
                <li>{line}</li>
              {/each}
            </ul>
          </div>
        {/if}
      </div>

      <footer
        class="flex items-center justify-between border-t border-blender-border bg-blender-card px-6 py-4"
      >
        <span class="text-xs text-zinc-400">
          Métodos gerados por tabela: <code class="text-zinc-200">findAll</code>, <code class="text-zinc-200">insert</code>, <code class="text-zinc-200">update</code>, <code class="text-zinc-200">delete</code>
        </span>
        <div class="flex gap-3">
          <button
            type="button"
            class={LocalStyleComponent.migration.ghostButton}
            onclick={onclose}
            disabled={generating}
          >
            Fechar
          </button>
          <button
            type="submit"
            class={LocalStyleComponent.migration.primaryButton}
            disabled={generating || selectedTableNames.length === 0}
          >
            {generating ? "A gerar projecto…" : "Gerar Aplicação CRUD"}
          </button>
        </div>
      </footer>
    </form>
  </div>
</div>

