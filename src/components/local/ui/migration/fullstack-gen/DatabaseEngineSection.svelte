<script lang="ts">
  import type { DatabaseEngine, FrontendFramework } from "../../../data/migration";

  let {
    databaseEngine = $bindable<DatabaseEngine>("postgres"),
    frontendFramework = "sveltekit",
  }: { databaseEngine?: DatabaseEngine; frontendFramework?: FrontendFramework } = $props();
</script>

<section class="space-y-3 rounded-xl border border-blender-border bg-blender-base/50 p-4">
  <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-300">4. Motor de Base de Dados para Conexão Direta nas Rotas de API ({frontendFramework})</h3>
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
