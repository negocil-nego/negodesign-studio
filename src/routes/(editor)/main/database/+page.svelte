<script lang="ts">
  import {
    databaseTables,
    type DatabaseTable,
  } from "../../../../components/local/data/migration";
  import MigrationEmptyState from "../../../../components/local/ui/migration/MigrationEmptyState.svelte";
  import ComponentizeDialog from "../../../../components/local/ui/migration/ComponentizeDialog.svelte";
  import { LocalStyleComponent } from "../../../../components/local/data/style.component";

  let selectedTable = $state<DatabaseTable | null>(null);
</script>

<svelte:head>
  <title>Base de dados · Negodesign</title>
  <meta
    name="description"
    content="Estrutura das tabelas importadas da base de dados."
  />
</svelte:head>

<div class="h-full w-full overflow-y-auto">
  <div class="mx-auto flex w-full max-w-9xl flex-col gap-6 px-5 py-8 sm:px-4">
    <header>
      <p
        class="text-xs font-semibold uppercase tracking-[0.18em] text-blue-500"
      >
        Migração
      </p>
      <h1
        class="mt-2 text-3xl font-semibold tracking-tight text-zinc-800 dark:text-zinc-100"
      >
        Estrutura da base de dados
      </h1>
      <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-400">
        {$databaseTables.length} tabela(s) importada(s).
      </p>
    </header>

    {#if $databaseTables.length}
      <div class="grid gap-6 lg:grid-cols-2 xl:grid-cols-3">
        {#each $databaseTables as table (table.name)}
          <article class={LocalStyleComponent.migration.card}>
            <header
              class="flex items-center justify-between gap-3 border-b border-blender-border px-6 py-4"
            >
              <div class="min-w-0 flex-1">
                <h2
                  class="truncate font-mono text-base font-semibold text-zinc-800 dark:text-zinc-100"
                  title={table.name}
                >
                  {table.name}
                </h2>
                <span class="text-xs text-zinc-500"
                  >{table.columns.length} coluna(s)</span
                >
              </div>
              <button
                type="button"
                class="shrink-0 rounded bg-blue-600 px-4 py-2 text-xs font-semibold text-white transition hover:bg-blue-500"
                onclick={() => (selectedTable = table)}
              >
                Componentizar
              </button>
            </header>
            <div class="max-h-[500px] overflow-y-auto overflow-x-auto">
              <table class="w-full text-left text-xs">
                <thead class={LocalStyleComponent.migration.cellHead}>
                  <tr class="sticky top-0 z-10">
                    <th class="bg-blender-base px-4 py-2 font-semibold"
                      >Coluna</th
                    >
                    <th class="bg-blender-base px-4 py-2 font-semibold">Tipo</th
                    >
                    <th class="bg-blender-base px-4 py-2 font-semibold">Nulo</th
                    >
                    <th class="bg-blender-base px-4 py-2 font-semibold"
                      >Chave</th
                    >
                  </tr>
                </thead>
                <tbody>
                  {#each table.columns as column (column.name)}
                    <tr class="border-t border-blender-border/60">
                      <td
                        class="px-4 py-2 font-mono text-zinc-700 dark:text-zinc-200"
                        >{column.name}</td
                      >
                      <td class="px-4 py-2 text-zinc-500">{column.dataType}</td>
                      <td class="px-4 py-2 text-zinc-500"
                        >{column.nullable ? "Sim" : "Não"}</td
                      >
                      <td class="px-4 py-2 text-zinc-500"
                        >{column.primaryKey ? "PK" : "—"}</td
                      >
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
</div>

{#if selectedTable}
  <ComponentizeDialog
    table={selectedTable}
    onclose={() => (selectedTable = null)}
  />
{/if}
