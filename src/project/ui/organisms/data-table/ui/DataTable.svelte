<script lang="ts">
  import Icon from "../../../atoms/icon/ui/Icon.svelte";
  import Text from "../../../atoms/text/ui/Text.svelte";
  import { StyleComponent } from "../../../../data/style.component";
  import { dataTableExampleByVariant } from "../data/examples";
  import DataTableFilter from "../shared/data-table-filter/ui/DataTableFilter.svelte";
  import type { DataTableProps, DataTableVariant } from "../data/types";
  import type { TextProps } from "../../../atoms/text/data/types";

  let {
    variant = "01",
    title,
    columns: suppliedColumns,
    rows: suppliedRows,
    className = "",
  }: DataTableProps = $props();

  const S = StyleComponent.dataTable;

  let searchQuery = $state("");
  let categoryFilter = $state("all");
  let menuRowIndex = $state<number | null>(null);
  let sortAsc = $state(true);
  let selected = $state<number[]>([]);
  let hiddenColumns = $state<number[]>([]);

  const example = $derived(
    dataTableExampleByVariant[variant as DataTableVariant] ?? dataTableExampleByVariant["01"],
  );

  const columns = $derived<TextProps[]>(suppliedColumns ?? example.columns);

  const allChunkedRows = $derived.by(() => {
    const flat = suppliedRows ?? example.rows;
    const n = columns.length || 1;
    const chunked: TextProps[][] = [];
    for (let i = 0; i < flat.length; i += n) chunked.push(flat.slice(i, i + n));
    return chunked;
  });

  const filterOptions = $derived.by(() => {
    const colIdx = columns.length > 1 ? 1 : 0;
    const values = new Set<string>();
    for (const row of allChunkedRows) {
      const val = row[colIdx]?.text?.trim();
      if (val) values.add(val);
    }
    return [...values];
  });

  const rows = $derived.by(() => {
    let list = sortAsc ? [...allChunkedRows] : [...allChunkedRows].reverse();

    if (categoryFilter !== "all") {
      const colIdx = columns.length > 1 ? 1 : 0;
      list = list.filter((row) => row[colIdx]?.text === categoryFilter);
    }

    if (searchQuery.trim()) {
      const q = searchQuery.toLowerCase();
      list = list.filter((row) => row.some((cell) => cell.text.toLowerCase().includes(q)));
    }

    return list;
  });

  function toggleCol(i: number) {
    if (i === 0) return;
    hiddenColumns = hiddenColumns.includes(i)
      ? hiddenColumns.filter((x) => x !== i)
      : [...hiddenColumns, i];
  }

  function toggleSelected(i: number) {
    selected = selected.includes(i) ? selected.filter((x) => x !== i) : [...selected, i];
  }

  function resetFilters() {
    searchQuery = "";
    categoryFilter = "all";
    hiddenColumns = [];
  }
</script>

<section
  class={`${S.root} ${className}`}
  aria-label={title ?? `Tabela de dados, variação ${variant}`}
>
  {#if title}<h3 class={S.title}>{title}</h3>{/if}

  <DataTableFilter
    {columns}
    {filterOptions}
    {searchQuery}
    {categoryFilter}
    {hiddenColumns}
    onSearchChange={(val) => (searchQuery = val)}
    onCategoryChange={(val) => (categoryFilter = val)}
    onToggleColumn={toggleCol}
    onReset={resetFilters}
  />

  <div class={S.viewport}>
    <table class={`${S.table} ${variant === "02" ? S.tableCompact : ""}`}>
      <thead class={S.head}>
        <tr>
          {#if variant !== "01"}
            <th class={S.checkboxCell}>
              <input type="checkbox" class={S.checkbox} aria-label="Seleccionar todas as linhas" />
            </th>
          {/if}
          {#each columns as col, ci (ci)}
            {#if !hiddenColumns.includes(ci)}
              <th class={`${S.headCell} ${col.className ?? ""}`}>
                <button
                  class={S.headCellButton}
                  onclick={() => (sortAsc = !sortAsc)}
                  aria-label={`Ordenar por ${col.text}`}
                >
                  <Text {...col} className="" />
                  <span aria-hidden="true" class={S.headCellSortIcon}>↕</span>
                </button>
              </th>
            {/if}
          {/each}
        </tr>
      </thead>

      <tbody>
        {#each rows as row, ri (ri)}
          <tr class={S.row}>
            {#if variant !== "01"}
              <td class="px-3 py-2">
                <input
                  type="checkbox"
                  class={S.checkbox}
                  checked={selected.includes(ri)}
                  aria-label={`Seleccionar ${row[0]?.text ?? ""}`}
                  onchange={() => toggleSelected(ri)}
                />
              </td>
            {/if}

            {#each columns as _col, ci (ci)}
              {#if !hiddenColumns.includes(ci)}
                {@const cell = row[ci] ?? { text: "" }}
                <td class={`${S.cell} ${variant === "02" ? S.cellCompact : ""}`}>
                  {#if variant === "03" && ci === 1}
                    <a href={`mailto:${cell.text}`} class="text-blue-400 underline underline-offset-2">
                      <Text {...cell} />
                    </a>
                  {:else if variant === "04" && ci === 7}
                    <span class="inline-flex gap-1">
                      <button class={S.actionButton}>Invoice</button>
                      <button class={S.actionButton}>Ledger</button>
                    </span>
                  {:else if variant === "01" && ci === 6}
                    <span class="relative inline-flex gap-1">
                      <button class={S.actionButton} aria-label="Ligar"><Icon name="Phone" /></button>
                      <button class={S.actionButton} aria-label="Adicionar"><Icon name="FolderAdd" /></button>
                      <button
                        class={S.actionButton}
                        aria-label="Mais acções"
                        onclick={() => (menuRowIndex = menuRowIndex === ri ? null : ri)}
                      ><Icon name="MoreHorizontal" /></button>
                      {#if menuRowIndex === ri}
                        <span class={S.rowMenu}>
                          {#each ["Details", "Offer a deal", "Add to my list", "Delete"] as action}
                            <button class={S.rowMenuItem}>{action}</button>
                          {/each}
                        </span>
                      {/if}
                    </span>
                  {:else if variant === "03" && ci === 6}
                    <span class="inline-flex gap-1">
                      <button class={S.actionButton}>Edit</button>
                      <button class={S.actionButton}>Delete</button>
                    </span>
                  {:else}
                    <span class={variant === "02" && ci === 0 ? "font-semibold text-zinc-100" : "text-zinc-200"}>
                      <Text {...cell} />
                    </span>
                  {/if}
                </td>
              {/if}
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  {#if variant !== "01" && variant !== "05"}
    <footer class={S.footer}>
      <span>{variant === "02" ? "Page 1 of 13" : "Page 1 of 8"}</span>
      <div class={S.footerGroup}>
        <label class="flex items-center gap-2">
          Rows per page
          <select class={S.footerSelect} aria-label="Linhas por página">
            <option>10</option>
            <option>20</option>
          </select>
        </label>
        <button class={S.footerButton} aria-label="Página anterior">‹</button>
        <button class={S.footerButton} aria-label="Página seguinte">›</button>
      </div>
    </footer>
  {/if}
</section>
