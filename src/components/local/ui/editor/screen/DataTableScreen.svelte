<script lang="ts">
  import DataTable from "../../../../../project/ui/organisms/data-table/ui/DataTable.svelte";
  import { dataTableExamples } from "../../../../../project/ui/organisms/data-table/data/examples";
  import SourceCode from "../terminal/SourceCode.svelte";
  import Terminal from "../terminal/Terminal.svelte";

  function svelteUsage(example: (typeof dataTableExamples)[number]) {
    return `<script lang="ts">\n  import DataTable from "./DataTable.svelte";\n  const columns = ${JSON.stringify(example.columns, null, 2)};\n  const rows = ${JSON.stringify(example.rows, null, 2)};\n<${"/scr" + "ipt"}>\n\n<DataTable variant="${example.variant}" {columns} {rows} />`;
  }
  function reactUsage(example: (typeof dataTableExamples)[number]) {
    return `import DataTable from "./DataTable";\n\nconst columns = ${JSON.stringify(example.columns, null, 2)};\nconst rows = ${JSON.stringify(example.rows, null, 2)};\n\nexport function Example() {\n  return <DataTable variant="${example.variant}" columns={columns} rows={rows} />;\n}`;
  }
  function vueUsage(example: (typeof dataTableExamples)[number]) {
    return `<script setup lang="ts">\nimport DataTable from "./DataTable.vue";\nconst columns = ${JSON.stringify(example.columns, null, 2)};\nconst rows = ${JSON.stringify(example.rows, null, 2)};\n<${"/scr" + "ipt"}>\n\n<template>\n  <DataTable variant="${example.variant}" :columns="columns" :rows="rows" />\n<${"/tem" + "plate"}>`;
  }
  function flutterUsage(example: (typeof dataTableExamples)[number]) {
    const columns = example.columns.map((column) => `DataTableColumn(label: '${column.label}', key: '${column.key}')`).join(",\n  ");
    const rows = example.rows.map((row) => `DataTableRowData(id: '${row.id}', cells: ${JSON.stringify(row.cells)},${row.avatar ? ` avatar: '${row.avatar}',` : ""}${row.badge ? ` badge: '${row.badge}',` : ""})`).join(",\n  ");
    return `import 'DataTable.dart';\nimport '../data/types.dart';\n\nfinal columns = <DataTableColumn>[\n  ${columns}\n];\nfinal rows = <DataTableRowData>[\n  ${rows}\n];\n\nDataTableWidget(variant: '${example.variant}', columns: columns, rows: rows);`;
  }
</script>

<div class="space-y-5">
  <div>
    <p class="text-[10px] font-semibold uppercase tracking-[0.18em] text-blue-400">Organismos / Data Table</p>
    <h2 class="mt-1 text-lg font-semibold text-zinc-100">Data tables</h2>
    <p class="mt-2 text-xs leading-5 text-zinc-400">Cada tabela recebe as suas colunas e linhas e inclui exemplos de utilização nas quatro linguagens.</p>
  </div>

  <div class="grid min-w-0 grid-cols-1 gap-4">
    {#each dataTableExamples as example (example.variant)}
      <Terminal id={`data-table-${example.variant}`} name={`DataTable · ${example.label}`} description="Tabela responsiva com colunas e linhas fornecidas pelo projecto.">
        {#snippet preview()}
          <div class="rounded-lg bg-blender-base p-4">
            <DataTable variant={example.variant} columns={example.columns} rows={example.rows} />
          </div>
        {/snippet}
        {#snippet svelte()}
          <SourceCode path={`Uso · DataTable ${example.variant} · Svelte`} source={svelteUsage(example)} />
        {/snippet}
        {#snippet react()}
          <SourceCode path={`Uso · DataTable ${example.variant} · React`} source={reactUsage(example)} />
        {/snippet}
        {#snippet vue()}
          <SourceCode path={`Uso · DataTable ${example.variant} · Vue`} source={vueUsage(example)} />
        {/snippet}
        {#snippet flutter()}
          <SourceCode path={`Uso · DataTable ${example.variant} · Flutter`} source={flutterUsage(example)} />
        {/snippet}
      </Terminal>
    {/each}
  </div>
</div>
