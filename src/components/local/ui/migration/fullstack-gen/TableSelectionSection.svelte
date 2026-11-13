<script lang="ts">
  import type { DatabaseTable } from "../../../data/migration";
  import { selectAllTableNames, toggleTableName } from "./generate";

  let {
    tables = [],
    selectedTableNames = $bindable<string[]>([]),
  }: { tables?: DatabaseTable[]; selectedTableNames?: string[] } = $props();

  const toggleTable = (name: string) =>
    (selectedTableNames = toggleTableName(selectedTableNames, name));
  const selectAllTables = () => (selectedTableNames = selectAllTableNames(tables));
  const clearSelectedTables = () => (selectedTableNames = []);
</script>

<!-- 2. Selecção de Tabelas -->
<section class="space-y-3 rounded-xl border border-blender-border bg-blender-base/50 p-4">
  <div class="flex flex-wrap items-center justify-between gap-2">
    <div>
      <h3 class="text-xs font-semibold uppercase tracking-wider text-zinc-300">2. Tabelas a Gerar ({selectedTableNames.length} de {tables.length})</h3>
      <p class="text-[11px] text-zinc-500">Escolha todas as tabelas ou apenas algumas para gerar os endpoints e o menu lateral do `AdminCrud`.</p>
    </div>
    <div class="flex gap-2">
      <button type="button" class="rounded border border-blender-border bg-blender-card px-2.5 py-1 text-[11px] text-zinc-300 hover:bg-blender-hover" onclick={selectAllTables}>Seleccionar todas</button>
      <button type="button" class="rounded border border-blender-border bg-blender-card px-2.5 py-1 text-[11px] text-zinc-400 hover:bg-blender-hover" onclick={clearSelectedTables}>Limpar</button>
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
