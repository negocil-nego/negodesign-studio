<script lang="ts">
  import { createSpringBootProject, type SpringBootProjectInput, type SpringBootProjectResult } from "./paths";
  import { LocalStyleComponent } from "../../../data/style.component";
  import type { DatabaseEngine, SpringLanguage } from "../../../data/migration";

  let {
    targetDir,
    language,
    ormStrategy,
    databaseEngine,
    onclose,
    oncreated,
  }: {
    targetDir: string;
    language: SpringLanguage;
    ormStrategy: string;
    databaseEngine: DatabaseEngine;
    onclose: () => void;
    oncreated: (projectDir: string) => void;
  } = $props();

  let groupId = $state("com.negocil.negoturismo");
  let artifactId = $state("admin-api");
  let name = $state("Admin API");
  let description = $state("API REST gerada pelo Negodesign");
  let includeApiDocs = $state(true);
  let creating = $state(false);
  let error = $state<string | null>(null);
  let result = $state<SpringBootProjectResult | null>(null);

  const isJpa = ormStrategy !== "spring-jdbc" && ormStrategy !== "jdbc";

  const ormInfo = isJpa
    ? {
        title: "Spring Data JPA",
        desc: "Mapeia tabelas para entidades Jakarta Persistence e usa JpaRepository com paginação. Ideal para relações complexas e consultas derivadas.",
      }
    : {
        title: "Spring JDBC (JdbcClient)",
        desc: "Acesso direto à base de dados com a nova API fluente JdbcClient. Mais leve e sem overhead de ORM, ideal para queries simples.",
      };

  const dbInfo =
    databaseEngine === "postgres"
      ? { title: "PostgreSQL", desc: "Driver JDBC para PostgreSQL. URL: jdbc:postgresql://localhost:5432/negodesign" }
      : databaseEngine === "mysql"
        ? { title: "MySQL", desc: "Driver JDBC para MySQL. URL: jdbc:mysql://localhost:3306/negodesign" }
        : { title: "SQLite", desc: "Driver JDBC para SQLite. Base de dados em ficheiro local." };

  const dependencies = [
    {
      name: "spring-boot-starter-web",
      desc: "API REST com Spring MVC — controladores, validação e serialização JSON.",
      required: true,
    },
    {
      name: isJpa ? "spring-boot-starter-data-jpa" : "spring-boot-starter-jdbc",
      desc: ormInfo.desc,
      required: true,
    },
    {
      name: databaseEngine === "postgres" ? "postgresql" : databaseEngine === "mysql" ? "mysql-connector-j" : "sqlite-jdbc",
      desc: dbInfo.desc,
      required: true,
    },
    {
      name: "springdoc-openapi-starter-webmvc-ui",
      desc: "Gera automaticamente documentação OpenAPI 3 e interface Swagger UI em /swagger-ui.html.",
      required: false,
      toggled: includeApiDocs,
    },
  ];

  async function handleCreate() {
    error = null;
    creating = true;
    try {
      const input: SpringBootProjectInput = {
        targetDir,
        groupId: groupId.trim(),
        artifactId: artifactId.trim(),
        name: name.trim(),
        description: description.trim(),
        language,
        ormStrategy,
        databaseEngine,
        includeApiDocs,
      };
      result = await createSpringBootProject(input);
      oncreated(result.projectDir);
    } catch (e) {
      error = String(e);
    } finally {
      creating = false;
    }
  }
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4" onclick={onclose}>
  <div
    class="max-h-[90vh] w-full max-w-2xl overflow-y-auto rounded-2xl border border-blender-border bg-blender-card p-6 shadow-2xl"
    onclick={(e) => e.stopPropagation()}
  >
    <div class="flex items-start justify-between gap-4">
      <div>
        <h2 class="text-xl font-semibold text-zinc-100">Criar Projecto Spring Boot</h2>
        <p class="mt-1 text-sm text-zinc-400">
          Vai ser criado um projecto Maven com as dependências selecionadas.
        </p>
      </div>
      <button class="text-zinc-400 hover:text-zinc-200" onclick={onclose}>✕</button>
    </div>

    {#if result}
      <div class="mt-5 space-y-4">
        <div class="rounded-lg border border-emerald-500/40 bg-emerald-500/10 p-4">
          <p class="font-semibold text-emerald-300">Projecto criado com sucesso!</p>
          <p class="mt-1 text-sm text-zinc-300">{result.projectDir}</p>
          <p class="mt-2 text-xs text-zinc-400">{result.generatedFiles} ficheiros gerados · {result.buildTool} · Spring Boot {result.springBootVersion}</p>
        </div>

        <div>
          <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-300">Dependências incluídas</h3>
          <ul class="mt-2 space-y-1 text-sm text-zinc-300">
            {#each result.dependencies as dep}
              <li class="font-mono text-xs">• {dep}</li>
            {/each}
          </ul>
        </div>

        <div>
          <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-300">Próximos passos</h3>
          <ol class="mt-2 list-decimal space-y-1 pl-5 text-sm text-zinc-300">
            {#each result.nextSteps as step}
              <li class="font-mono text-xs">{step}</li>
            {/each}
          </ol>
        </div>

        <div class="flex justify-end gap-2">
          <button class="rounded-lg bg-blue-600 px-4 py-2 text-sm font-semibold text-white hover:bg-blue-500" onclick={onclose}>
            Usar este caminho
          </button>
        </div>
      </div>
    {:else}
      <div class="mt-5 space-y-5">
        <div class="grid gap-4 sm:grid-cols-2">
          <label class="block">
            <span class={LocalStyleComponent.migration.label}>Group ID</span>
            <input class={LocalStyleComponent.migration.field} bind:value={groupId} placeholder="com.exemplo" />
          </label>
          <label class="block">
            <span class={LocalStyleComponent.migration.label}>Artifact ID</span>
            <input class={LocalStyleComponent.migration.field} bind:value={artifactId} placeholder="admin-api" />
          </label>
        </div>

        <div class="grid gap-4 sm:grid-cols-2">
          <label class="block">
            <span class={LocalStyleComponent.migration.label}>Nome do projecto</span>
            <input class={LocalStyleComponent.migration.field} bind:value={name} />
          </label>
          <label class="block">
            <span class={LocalStyleComponent.migration.label}>Descrição</span>
            <input class={LocalStyleComponent.migration.field} bind:value={description} />
          </label>
        </div>

        <div class="rounded-lg border border-blender-border bg-blender-base/50 p-4">
          <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-300">Configuração detectada</h3>
          <div class="mt-3 grid gap-2 text-sm">
            <div class="flex justify-between"><span class="text-zinc-400">Linguagem</span><span class="font-medium text-zinc-200">{language === "kotlin" ? "Kotlin" : "Java"}</span></div>
            <div class="flex justify-between"><span class="text-zinc-400">ORM</span><span class="font-medium text-zinc-200">{ormInfo.title}</span></div>
            <div class="flex justify-between"><span class="text-zinc-400">Base de dados</span><span class="font-medium text-zinc-200">{dbInfo.title}</span></div>
          </div>
        </div>

        <div>
          <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-300">Dependências que serão incluídas</h3>
          <div class="mt-3 space-y-2">
            {#each dependencies as dep}
              <label class="flex items-start gap-3 rounded-lg border border-blender-border bg-blender-base/50 p-3">
                {#if dep.required}
                  <input type="checkbox" checked disabled class="mt-0.5 accent-blue-500" />
                {:else}
                  <input type="checkbox" bind:checked={includeApiDocs} class="mt-0.5 accent-blue-500" />
                {/if}
                <div class="min-w-0 flex-1">
                  <div class="flex items-center gap-2">
                    <span class="font-mono text-xs font-semibold text-zinc-200">{dep.name}</span>
                    {#if dep.required}
                      <span class="rounded bg-blue-500/20 px-1.5 py-0.5 text-[9px] font-semibold text-blue-300">OBRIGATÓRIO</span>
                    {/if}
                  </div>
                  <p class="mt-1 text-xs text-zinc-400">{dep.desc}</p>
                </div>
              </label>
            {/each}
          </div>
        </div>

        {#if error}
          <div class="rounded-lg border border-red-500/40 bg-red-500/10 p-3 text-sm text-red-300">{error}</div>
        {/if}

        <div class="flex justify-end gap-2">
          <button class="rounded-lg border border-blender-border px-4 py-2 text-sm font-medium text-zinc-300 hover:bg-blender-hover" onclick={onclose}>
            Cancelar
          </button>
          <button
            class="rounded-lg bg-emerald-600 px-4 py-2 text-sm font-semibold text-white hover:bg-emerald-500 disabled:opacity-50"
            disabled={creating || !groupId.trim() || !artifactId.trim()}
            onclick={handleCreate}
          >
            {creating ? "A criar..." : "Criar projecto"}
          </button>
        </div>
      </div>
    {/if}
  </div>
</div>
