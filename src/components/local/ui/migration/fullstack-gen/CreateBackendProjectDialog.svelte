<script lang="ts">
  import { createSpringBootProject, createNestjsProject, createLaravelProject, createAspnetProject, type SpringBootProjectResult, type BackendProjectResult } from "./paths";
  import { LocalStyleComponent } from "../../../data/style.component";
  import type { BackendFramework, DatabaseEngine, SpringLanguage } from "../../../data/migration";

  let {
    targetDir,
    framework,
    language = "java",
    ormStrategy,
    databaseEngine,
    onclose,
    oncreated,
  }: {
    targetDir: string;
    framework: BackendFramework;
    language?: SpringLanguage;
    ormStrategy: string;
    databaseEngine: DatabaseEngine;
    onclose: () => void;
    oncreated: (projectDir: string) => void;
  } = $props();

  const isSpring = framework === "spring-boot";
  const isNest = framework === "nestjs";
  const isLaravel = framework === "laravel";
  const isAspnet = framework === "aspnet-core";

  const fwLabel = isSpring ? "Spring Boot" : isNest ? "NestJS" : isLaravel ? "Laravel" : "ASP.NET Core";
  const fwDesc = isSpring ? "Maven" : isNest ? "npm" : isLaravel ? "Composer" : "dotnet";
  const supportsApiDocs = !isLaravel;

  let name = $state("negodesign-api");
  let description = $state("API REST gerada pelo Negodesign");
  let groupId = $state("com.negodesign");
  let artifactId = $state("admin-api");
  let includeApiDocs = $state(true);
  let creating = $state(false);
  let error = $state<string | null>(null);
  let result = $state<BackendProjectResult | SpringBootProjectResult | null>(null);

  const dbLabel = databaseEngine === "postgres" ? "PostgreSQL" : databaseEngine === "mysql" ? "MySQL" : "SQLite";
  const ormLabel = isSpring
    ? ormStrategy === "spring-jdbc" ? "Spring JDBC" : "Spring Data JPA"
    : isNest
      ? ormStrategy === "prisma" ? "Prisma ORM" : ormStrategy === "drizzle" ? "Drizzle ORM" : "TypeORM"
      : isLaravel
        ? ormStrategy === "eloquent" ? "Eloquent ORM" : ormStrategy === "query-builder" ? "Query Builder" : "Raw SQL"
        : ormStrategy === "ef-core" ? "Entity Framework Core" : ormStrategy === "dapper" ? "Dapper" : "Raw SQL (ADO.NET)";

  const deps: { name: string; desc: string; required: boolean }[] = isSpring
    ? [
        { name: "spring-boot-starter-webmvc", desc: "API REST com Spring MVC — controladores, validação e JSON.", required: true },
        { name: ormStrategy === "spring-jdbc" ? "spring-boot-starter-jdbc" : "spring-boot-starter-data-jpa", desc: `${ormLabel} — persistência e repositórios.`, required: true },
        { name: databaseEngine === "postgres" ? "postgresql" : databaseEngine === "mysql" ? "mysql-connector-j" : "sqlite-jdbc", desc: `Driver JDBC para ${dbLabel}.`, required: true },
        { name: "springdoc-openapi-starter-webmvc-ui", desc: "Documentação OpenAPI 3 + Swagger UI em /swagger-ui.html.", required: false },
      ]
    : isNest
      ? [
          { name: "@nestjs/core + @nestjs/platform-express", desc: "Núcleo do NestJS com servidor Express.", required: true },
          { name: ormLabel, desc: `${ormLabel} para acesso à base de dados.`, required: true },
          { name: databaseEngine === "postgres" ? "pg" : databaseEngine === "mysql" ? "mysql2" : "better-sqlite3", desc: `Driver para ${dbLabel}.`, required: true },
          { name: "@nestjs/swagger", desc: "Documentação Swagger automática em /api.", required: false },
        ]
      : isLaravel
        ? [
            { name: "laravel/framework", desc: "Framework Laravel com Eloquent, routing e migrations.", required: true },
            { name: ormLabel, desc: `${ormLabel} integrado no Laravel.`, required: true },
            { name: `ext-pdo_${databaseEngine === "postgres" ? "pgsql" : databaseEngine === "mysql" ? "mysql" : "sqlite"}`, desc: `Extensão PHP para ${dbLabel}.`, required: true },
          ]
        : [
            { name: "Microsoft.AspNetCore.App", desc: "Framework ASP.NET Core Web API.", required: true },
            { name: ormLabel, desc: `${ormLabel} para acesso à base de dados.`, required: true },
            { name: databaseEngine === "postgres" ? "Npgsql.EntityFrameworkCore.PostgreSQL" : databaseEngine === "mysql" ? "MySql.EntityFrameworkCore" : "Microsoft.EntityFrameworkCore.Sqlite", desc: `Provider para ${dbLabel}.`, required: true },
            { name: "Swashbuckle.AspNetCore", desc: "Swagger UI em /swagger.", required: false },
          ];

  async function handleCreate() {
    error = null;
    creating = true;
    try {
      const base = {
        targetDir,
        name: name.trim(),
        description: description.trim(),
        ormStrategy,
        databaseEngine,
        includeApiDocs,
      };
      if (isSpring) {
        result = await createSpringBootProject({
          ...base,
          groupId: groupId.trim(),
          artifactId: artifactId.trim(),
          language,
        });
      } else if (isNest) {
        result = await createNestjsProject(base);
      } else if (isLaravel) {
        result = await createLaravelProject({ targetDir, name: name.trim(), description: description.trim(), ormStrategy, databaseEngine });
      } else {
        result = await createAspnetProject(base);
      }
      oncreated(result.projectDir);
    } catch (e) {
      error = String(e);
    } finally {
      creating = false;
    }
  }

  const canCreate = isSpring ? groupId.trim() && artifactId.trim() : name.trim();
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4" onclick={onclose}>
  <div class="max-h-[90vh] w-full max-w-2xl overflow-y-auto rounded-2xl border border-blender-border bg-blender-card p-6 shadow-2xl" onclick={(e) => e.stopPropagation()}>
    <div class="flex items-start justify-between gap-4">
      <div>
        <h2 class="text-xl font-semibold text-zinc-100">Criar Projecto {fwLabel}</h2>
        <p class="mt-1 text-sm text-zinc-400">Vai ser criado um projecto {fwDesc} com as dependências selecionadas.</p>
      </div>
      <button class="text-zinc-400 hover:text-zinc-200" onclick={onclose}>✕</button>
    </div>

    {#if result}
      <div class="mt-5 space-y-4">
        <div class="rounded-lg border border-emerald-500/40 bg-emerald-500/10 p-4">
          <p class="font-semibold text-emerald-300">Projecto criado com sucesso!</p>
          <p class="mt-1 text-sm text-zinc-300">{result.projectDir}</p>
          <p class="mt-2 text-xs text-zinc-400">{result.generatedFiles} ficheiros gerados · {result.buildTool}{"springBootVersion" in result ? ` · Spring Boot ${result.springBootVersion}` : ""}</p>
        </div>
        <div>
          <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-300">Dependências incluídas</h3>
          <ul class="mt-2 space-y-1 text-sm text-zinc-300">{#each result.dependencies as dep}<li class="font-mono text-xs">• {dep}</li>{/each}</ul>
        </div>
        <div>
          <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-300">Próximos passos</h3>
          <ol class="mt-2 list-decimal space-y-1 pl-5 text-sm text-zinc-300">{#each result.nextSteps as step}<li class="font-mono text-xs">{step}</li>{/each}</ol>
        </div>
        <div class="flex justify-end gap-2"><button class="rounded-lg bg-blue-600 px-4 py-2 text-sm font-semibold text-white hover:bg-blue-500" onclick={onclose}>Usar este caminho</button></div>
      </div>
    {:else}
      <div class="mt-5 space-y-5">
        {#if isSpring}
          <div class="grid gap-4 sm:grid-cols-2">
            <label class="block"><span class={LocalStyleComponent.migration.label}>Group ID</span><input class={LocalStyleComponent.migration.field} bind:value={groupId} placeholder="com.exemplo" /></label>
            <label class="block"><span class={LocalStyleComponent.migration.label}>Artifact ID</span><input class={LocalStyleComponent.migration.field} bind:value={artifactId} placeholder="admin-api" /></label>
          </div>
        {/if}
        <div class="grid gap-4 sm:grid-cols-2">
          <label class="block"><span class={LocalStyleComponent.migration.label}>Nome do projecto</span><input class={LocalStyleComponent.migration.field} bind:value={name} /></label>
          <label class="block"><span class={LocalStyleComponent.migration.label}>Descrição</span><input class={LocalStyleComponent.migration.field} bind:value={description} /></label>
        </div>

        <div class="rounded-lg border border-blender-border bg-blender-base/50 p-4">
          <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-300">Configuração detectada</h3>
          <div class="mt-3 grid gap-2 text-sm">
            {#if isSpring}<div class="flex justify-between"><span class="text-zinc-400">Linguagem</span><span class="font-medium text-zinc-200">{language === "kotlin" ? "Kotlin" : "Java"}</span></div>{/if}
            <div class="flex justify-between"><span class="text-zinc-400">ORM</span><span class="font-medium text-zinc-200">{ormLabel}</span></div>
            <div class="flex justify-between"><span class="text-zinc-400">Base de dados</span><span class="font-medium text-zinc-200">{dbLabel}</span></div>
          </div>
        </div>

        <div>
          <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-300">Dependências que serão incluídas</h3>
          <div class="mt-3 space-y-2">
            {#each deps as dep}
              <label class="flex items-start gap-3 rounded-lg border border-blender-border bg-blender-base/50 p-3">
                {#if dep.required}<input type="checkbox" checked disabled class="mt-0.5 accent-blue-500" />
                {:else if supportsApiDocs}<input type="checkbox" bind:checked={includeApiDocs} class="mt-0.5 accent-blue-500" />
                {/if}
                <div class="min-w-0 flex-1">
                  <div class="flex items-center gap-2">
                    <span class="font-mono text-xs font-semibold text-zinc-200">{dep.name}</span>
                    {#if dep.required}<span class="rounded bg-blue-500/20 px-1.5 py-0.5 text-[9px] font-semibold text-blue-300">OBRIGATÓRIO</span>{/if}
                  </div>
                  <p class="mt-1 text-xs text-zinc-400">{dep.desc}</p>
                </div>
              </label>
            {/each}
          </div>
        </div>

        {#if error}<div class="rounded-lg border border-red-500/40 bg-red-500/10 p-3 text-sm text-red-300">{error}</div>{/if}

        <div class="flex justify-end gap-2">
          <button class="rounded-lg border border-blender-border px-4 py-2 text-sm font-medium text-zinc-300 hover:bg-blender-hover" onclick={onclose}>Cancelar</button>
          <button class="rounded-lg bg-emerald-600 px-4 py-2 text-sm font-semibold text-white hover:bg-emerald-500 disabled:opacity-50" disabled={creating || !canCreate} onclick={handleCreate}>{creating ? "A criar..." : "Criar projecto"}</button>
        </div>
      </div>
    {/if}
  </div>
</div>
