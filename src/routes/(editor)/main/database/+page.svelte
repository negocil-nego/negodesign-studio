<script lang="ts">
  import { databaseTables } from "../../../../components/local/data/migration";
  import MigrationEmptyState from "../../../../components/local/ui/migration/MigrationEmptyState.svelte";
  import { StyleComponent } from "../../../../components/local/data/style.component";
</script>

<svelte:head>
  <title>Base de dados · Negodesign</title>
  <meta name="description" content="Estrutura das tabelas importadas da base de dados." />
</svelte:head>

<div class="mx-auto flex w-full max-w-7xl flex-col gap-6 px-5 py-8 sm:px-8">
  <header>
    <p class="text-xs font-semibold uppercase tracking-[0.18em] text-blue-500">Migração</p>
    <h1 class="mt-2 text-3xl font-semibold tracking-tight text-zinc-800 dark:text-zinc-100">
      Estrutura da base de dados
    </h1>
    <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-400">
      {$databaseTables.length} tabela(s) importada(s).
    </p>
  </header>

  {#if $databaseTables.length}
    <div class="grid gap-4 lg:grid-cols-2">
      {#each $databaseTables as table (table.name)}
        <article class={StyleComponent.migration.card}>
          <header class="flex items-center justify-between gap-3 border-b border-blender-border px-4 py-3">
            <h2 class="truncate font-mono text-sm font-semibold text-zinc-800 dark:text-zinc-100" title={table.name}>
              {table.name}
            </h2>
            <span class={StyleComponent.migration.badge}>{table.columns.length} coluna(s)</span>
          </header>
          <div class="overflow-x-auto">
            <table class="w-full text-left text-xs">
              <thead class={StyleComponent.migration.cellHead}>
                <tr>
                  <th class="px-4 py-2 font-semibold">Coluna</th>
                  <th class="px-4 py-2 font-semibold">Tipo</th>
                  <th class="px-4 py-2 font-semibold">Nulo</th>
                  <th class="px-4 py-2 font-semibold">Chave</th>
                </tr>
              </thead>
              <tbody>
                {#each table.columns as column (column.name)}
                  <tr class="border-t border-blender-border/60">
                    <td class="px-4 py-2 font-mono text-zinc-700 dark:text-zinc-200">{column.name}</td>
                    <td class="px-4 py-2 text-zinc-500">{column.dataType}</td>
                    <td class="px-4 py-2 text-zinc-500">{column.nullable ? "Sim" : "Não"}</td>
                    <td class="px-4 py-2 text-zinc-500">{column.primaryKey ? "PK" : "—"}</td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </article>
      {/each}
    </div>
  {:else}
    <MigrationEmptyState
      title="Ainda não há tabelas importadas"
      description="Abra «Novo Projecto» no menu lateral, escolha Migração de projecto → Banco de dados e ligue-se a um servidor MySQL ou PostgreSQL."
    />
  {/if}
</div>
