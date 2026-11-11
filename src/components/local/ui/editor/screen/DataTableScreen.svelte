<script lang="ts">
  import DataTable from "../../../../../project/ui/organisms/data-table/ui/DataTable.svelte";
  import { dataTableExamples } from "../../../../../project/ui/organisms/data-table/data/examples";
  import SourceCode from "../terminal/SourceCode.svelte";
  import Terminal from "../terminal/Terminal.svelte";

  function svelteUsage(example: (typeof dataTableExamples)[number]) {
    return `<script lang="ts">\n  import DataTable from "./DataTable.svelte";\n  import type { DataTextProps } from "../data/types";\n  const columns: DataTextProps[] = ${JSON.stringify(example.columns, null, 2)};\n  const rows: DataTextProps[] = ${JSON.stringify(example.rows, null, 2)};\n<${"/scr" + "ipt"}>\n\n<DataTable variant="${example.variant}" {columns} {rows} />`;
  }
  function reactUsage(example: (typeof dataTableExamples)[number]) {
    return `import DataTable from "./DataTable";\nimport type { DataTextProps } from "../data/types";\n\nconst columns: DataTextProps[] = ${JSON.stringify(example.columns, null, 2)};\nconst rows: DataTextProps[] = ${JSON.stringify(example.rows, null, 2)};\n\nexport function Example() {\n  return <DataTable variant="${example.variant}" columns={columns} rows={rows} />;\n}`;
  }
  function vueUsage(example: (typeof dataTableExamples)[number]) {
    return `<script setup lang="ts">\nimport DataTable from "./DataTable.vue";\nimport type { DataTextProps } from "../data/types";\nconst columns: DataTextProps[] = ${JSON.stringify(example.columns, null, 2)};\nconst rows: DataTextProps[] = ${JSON.stringify(example.rows, null, 2)};\n<${"/scr" + "ipt"}>\n\n<template>\n  <DataTable variant="${example.variant}" :columns="columns" :rows="rows" />\n<${"/tem" + "plate"}>`;
  }
  function flutterUsage(example: (typeof dataTableExamples)[number]) {
    const columns = example.columns
      .map((column) => `DataTextProps(text: '${column.text}'${column.type ? `, type: '${column.type}'` : ""})`)
      .join(",\n  ");
    const rows = example.rows
      .map(
        (row) =>
          `DataTextProps(text: '${row.text}'${row.type ? `, type: '${row.type}'` : ""}${row.badgeVariant ? `, badgeVariant: '${row.badgeVariant}'` : ""})`,
      )
      .join(",\n  ");
    return `import 'DataTable.dart';\nimport '../data/types.dart';\n\nfinal columns = <DataTextProps>[\n  ${columns}\n];\nfinal rows = <DataTextProps>[\n  ${rows}\n];\n\nDataTableWidget(variant: '${example.variant}', columns: columns, rows: rows);`;
  }
</script>

<div class="space-y-5">
  <div>
    <p class="text-[10px] font-semibold uppercase tracking-[0.18em] text-blue-400">Organismos / Data Table</p>
    <h2 class="mt-1 text-lg font-semibold text-zinc-100">Data tables</h2>
    <p class="mt-2 text-xs leading-5 text-zinc-400">Cada tabela recebe as suas colunas e linhas (`DataTextProps`) com suporte a badges, toggles e dropdown de acções nas quatro linguagens.</p>
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
