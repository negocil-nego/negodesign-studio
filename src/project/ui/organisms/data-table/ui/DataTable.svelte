<script lang="ts">
  import Icon from "../../../atoms/icon/ui/Icon.svelte";
  import Text from "../../../atoms/text/ui/Text.svelte";
  import { StyleComponent } from "../../../../data/style.component";
  import { computeDataTableView, DATA_TABLE_ROW_ACTIONS, getCellKind, toggleNumberInList } from "../../../../hooks/data-table.hook";
  import DataTableFilter from "../shared/data-table-filter/ui/DataTableFilter.svelte";
  import type { DataTableProps, DataTableVariant } from "../data/types";

  let { variant = "01", title, columns: suppliedColumns, rows: suppliedRows, className = "" }: DataTableProps = $props();
  const S = StyleComponent.dataTable;

  let searchQuery = $state("");
  let categoryFilter = $state("all");
  let menuRowIndex = $state<number | null>(null);
  let sortAsc = $state(true);
  let selected = $state<number[]>([]);
  let hiddenColumns = $state<number[]>([]);

  const view = $derived(
    computeDataTableView({ variant: variant as DataTableVariant, columns: suppliedColumns, rows: suppliedRows, searchQuery, categoryFilter, sortAsc, hiddenColumns }),
  );
</script>

<section class={`${S.root} ${className}`} aria-label={title ?? `Data table ${variant}`}>
  {#if title}<h3 class={S.title}>{title}</h3>{/if}
  <DataTableFilter
    columns={view.columns}
    filterOptions={view.filterOptions}
    {searchQuery}
    {categoryFilter}
    {hiddenColumns}
    onSearchChange={(v) => (searchQuery = v)}
    onCategoryChange={(v) => (categoryFilter = v)}
    onToggleColumn={(ci) => (hiddenColumns = toggleNumberInList(hiddenColumns, ci, true))}
    onReset={() => { searchQuery = ""; categoryFilter = "all"; hiddenColumns = []; }}
  />
  <div class={S.viewport}>
    <table class={`${S.table} ${variant === "02" ? S.tableCompact : ""}`}>
      <thead class={S.head}>
        <tr>
          {#if view.showCheckbox}<th class={S.checkboxCell}><input type="checkbox" class={S.checkbox} aria-label="Select all" /></th>{/if}
          {#each view.columns as col, ci (ci)}
            {#if !hiddenColumns.includes(ci)}
              <th class={`${S.headCell} ${col.className ?? ""}`}>
                <button class={S.headCellButton} onclick={() => (sortAsc = !sortAsc)}><Text {...col} className="" /><span aria-hidden="true" class={S.headCellSortIcon}>↕</span></button>
              </th>
            {/if}
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each view.visibleRows as row, ri (ri)}
          <tr class={S.row}>
            {#if view.showCheckbox}
              <td class="px-3 py-2"><input type="checkbox" class={S.checkbox} checked={selected.includes(ri)} onchange={() => (selected = toggleNumberInList(selected, ri))} /></td>
            {/if}
            {#each view.columns as _c, ci (ci)}
              {#if !hiddenColumns.includes(ci)}
                {@const cell = row[ci] ?? { text: "" }}
                {@const kind = getCellKind(variant as DataTableVariant, ci)}
                <td class={`${S.cell} ${variant === "02" ? S.cellCompact : ""}`}>
                  {#if kind === "email"}
                    <a href={`mailto:${cell.text}`} class="text-blue-400 underline underline-offset-2"><Text {...cell} /></a>
                  {:else if kind === "invoice" || kind === "edit"}
                    <span class="inline-flex gap-1"><button class={S.actionButton}>{kind === "invoice" ? "Invoice" : "Edit"}</button><button class={S.actionButton}>{kind === "invoice" ? "Ledger" : "Delete"}</button></span>
                  {:else if kind === "menu"}
                    <span class="relative inline-flex gap-1">
                      <button class={S.actionButton} aria-label="Call"><Icon name="Phone" /></button>
                      <button class={S.actionButton} aria-label="Add"><Icon name="FolderAdd" /></button>
                      <button class={S.actionButton} onclick={() => (menuRowIndex = menuRowIndex === ri ? null : ri)}><Icon name="MoreHorizontal" /></button>
                      {#if menuRowIndex === ri}<span class={S.rowMenu}>{#each DATA_TABLE_ROW_ACTIONS as act}<button class={S.rowMenuItem}>{act}</button>{/each}</span>{/if}
                    </span>
                  {:else}
                    <span class={variant === "02" && ci === 0 ? "font-semibold text-zinc-100" : "text-zinc-200"}><Text {...cell} /></span>
                  {/if}
                </td>
              {/if}
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
  {#if view.showFooter}
    <footer class={S.footer}>
      <span>{view.pageLabel}</span>
      <div class={S.footerGroup}>
        <label class="flex items-center gap-2">Rows per page <select class={S.footerSelect}><option>10</option><option>20</option></select></label>
        <button class={S.footerButton}>‹</button><button class={S.footerButton}>›</button>
      </div>
    </footer>
  {/if}
</section>
