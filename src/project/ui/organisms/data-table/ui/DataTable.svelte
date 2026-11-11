<script lang="ts">
  import Icon from "../../../atoms/icon/ui/Icon.svelte";
  import Text from "../../../atoms/text/ui/Text.svelte";
  import { StyleComponent } from "../../../../data/style.component";
  import {
    computeDataTableView,
    getCellKind,
    normalizeActionItems,
    resolveBadgeVariantClass,
    toggleNumberInList,
  } from "../../../../hooks/data-table.hook";
  import DataTableFilter from "../shared/data-table-filter/ui/DataTableFilter.svelte";
  import type { DataTableProps, DataTableVariant, DataTextProps } from "../data/types";

  let {
    variant = "01",
    title,
    columns: suppliedColumns,
    rows: suppliedRows,
    className = "",
    actions,
    actionColumnLabel,
    showActionColumn,
    onRowAction,
    addLabel,
    onAdd,
  }: DataTableProps = $props();

  const S = StyleComponent.dataTable;

  let searchQuery = $state("");
  let categoryFilter = $state("all");
  let menuCellKey = $state<string | null>(null);
  let sortAsc = $state(true);
  let selected = $state<number[]>([]);
  let hiddenColumns = $state<number[]>([]);
  let toggleStates = $state<Record<string, boolean>>({});

  const view = $derived(
    computeDataTableView({
      variant: variant as DataTableVariant,
      columns: suppliedColumns,
      rows: suppliedRows,
      actions,
      actionColumnLabel,
      showActionColumn,
      searchQuery,
      categoryFilter,
      sortAsc,
      hiddenColumns,
    }),
  );

  function handleActionSelect(actionVal: string, ri: number, row: DataTextProps[]) {
    menuCellKey = null;
    onRowAction?.(actionVal, ri, row);
  }
</script>

{#if menuCellKey}
  <div class={S.overlay} role="presentation" onclick={() => (menuCellKey = null)}></div>
{/if}

<section class={`${S.root} ${className}`} aria-label={title ?? `Data table ${variant}`}>
  {#if title}<h3 class={S.title}>{title}</h3>{/if}
  <DataTableFilter
    columns={view.columns}
    filterOptions={view.filterOptions}
    {searchQuery}
    {categoryFilter}
    {hiddenColumns}
    {addLabel}
    {onAdd}
    onSearchChange={(v) => (searchQuery = v)}
    onCategoryChange={(v) => (categoryFilter = v)}
    onToggleColumn={(ci) => (hiddenColumns = toggleNumberInList(hiddenColumns, ci, true))}
    onReset={() => { searchQuery = ""; categoryFilter = "all"; hiddenColumns = []; }}
  />
  {#if selected.length > 0}
    <div class={S.selectionBar}>
      <span>{selected.length} selected</span>
      <button type="button" class="text-xs font-medium underline" onclick={() => (selected = [])}>Clear selection</button>
    </div>
  {/if}
  <div class={S.viewport}>
    <table class={`${S.table} ${variant === "02" ? S.tableCompact : ""}`}>
      <thead class={S.head}>
        <tr>
          {#if view.showCheckbox}
            <th class={S.checkboxCell}>
              <input
                type="checkbox"
                class={S.checkbox}
                aria-label="Select all"
                checked={view.visibleRows.length > 0 && selected.length === view.visibleRows.length}
                onchange={(e) => {
                  selected = (e.currentTarget as HTMLInputElement).checked
                    ? view.visibleRows.map((_, i) => i)
                    : [];
                }}
              />
            </th>
          {/if}
          {#each view.columns as col, ci (ci)}
            {#if !hiddenColumns.includes(ci)}
              <th class={`${S.headCell} ${col.className ?? ""}`}>
                <button type="button" class={S.headCellButton} onclick={() => (sortAsc = !sortAsc)}>
                  <Text text={col.text} className="" />
                  <span aria-hidden="true" class={S.headCellSortIcon}>↕</span>
                </button>
              </th>
            {/if}
          {/each}
        </tr>
      </thead>
      <tbody class={S.tbody}>
        {#each view.visibleRows as row, ri (ri)}
          <tr class={`${S.row} ${selected.includes(ri) ? S.trSelected : ""}`}>
            {#if view.showCheckbox}
              <td class="px-3 py-2">
                <input
                  type="checkbox"
                  class={S.checkbox}
                  checked={selected.includes(ri)}
                  onchange={() => (selected = toggleNumberInList(selected, ri))}
                />
              </td>
            {/if}
            {#each view.columns as col, ci (ci)}
              {#if !hiddenColumns.includes(ci)}
                {@const cell = row[ci] ?? { text: "" }}
                {@const kind = getCellKind(variant as DataTableVariant, ci, cell, col)}
                {@const cellId = `${ri}-${ci}`}
                <td class={`${S.cell} ${variant === "02" ? S.cellCompact : ""}`}>
                  {#if kind === "email"}
                    <a href={`mailto:${cell.text}`} class="text-blue-400 underline underline-offset-2"><Text {...cell} /></a>
                  {:else if kind === "badge"}
                    {@const badgeVariantCls = resolveBadgeVariantClass(cell.badgeVariant, cell.text, S.badge)}
                    <span class={cell.className ? cell.className : `${S.badge.base} ${badgeVariantCls}`}>
                      {#if cell.dot}<span class={S.badge.dot} aria-hidden="true"></span>{/if}
                      <span>{cell.text}</span>
                    </span>
                  {:else if kind === "toggle"}
                    {@const isChecked = toggleStates[cellId] ?? cell.checked ?? false}
                    <button
                      type="button"
                      role="switch"
                      aria-checked={isChecked}
                      aria-label={col.text || "Toggle"}
                      onclick={() => (toggleStates[cellId] = !isChecked)}
                      class={`relative inline-flex h-5 w-9 items-center rounded-full transition-colors ${isChecked ? "bg-blue-600" : "bg-zinc-700"}`}
                    >
                      <span class={`inline-block size-3.5 transform rounded-full bg-white transition-transform ${isChecked ? "translate-x-4" : "translate-x-1"}`}></span>
                    </button>
                  {:else if kind === "invoice" || kind === "edit"}
                    <span class="inline-flex gap-1">
                      <button
                        type="button"
                        class={S.actionButton}
                        onclick={() => handleActionSelect(kind === "invoice" ? "invoice" : "edit", ri, row)}
                      >
                        {kind === "invoice" ? "Invoice" : "Edit"}
                      </button>
                      <button
                        type="button"
                        class={S.actionButton}
                        onclick={() => handleActionSelect(kind === "invoice" ? "ledger" : "delete", ri, row)}
                      >
                        {kind === "invoice" ? "Ledger" : "Delete"}
                      </button>
                    </span>
                  {:else if kind === "dropdown"}
                    {@const rowActions = normalizeActionItems(cell, col, actions)}
                    <div class="relative inline-flex items-center gap-1">
                      <button
                        type="button"
                        class={S.menuButton}
                        aria-label="Row actions"
                        aria-expanded={menuCellKey === cellId}
                        onclick={() => (menuCellKey = menuCellKey === cellId ? null : cellId)}
                      >
                        {#if cell.text}
                          <span class="px-1.5 text-xs">{cell.text}</span>
                        {:else}
                          <Icon name="MoreHorizontal" />
                        {/if}
                      </button>
                      {#if menuCellKey === cellId}
                        <div class={S.rowMenu} role="menu">
                          {#each rowActions as act (act.label)}
                            <button
                              type="button"
                              role="menuitem"
                              class={act.variant === "danger" ? S.rowMenuDanger : S.rowMenuItem}
                              onclick={() => handleActionSelect(act.value ?? act.label, ri, row)}
                            >
                              {#if act.icon}<Icon name={act.icon} />{/if}
                              <span>{act.label}</span>
                            </button>
                          {/each}
                        </div>
                      {/if}
                    </div>
                  {:else}
                    <div class="flex items-center gap-2.5">
                      {#if cell.avatar}
                        <img src={cell.avatar} alt={cell.text} class="size-7 rounded-full object-cover" />
                      {/if}
                      <div>
                        <span class={variant === "02" && ci === 0 ? "font-semibold text-zinc-100" : "text-zinc-200"}>
                          <Text {...cell} />
                        </span>
                        {#if cell.subtitle}
                          <span class="block text-[11px] text-zinc-400">{cell.subtitle}</span>
                        {/if}
                      </div>
                    </div>
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
        <label class="flex items-center gap-2">Rows per page <select class={S.footerSelect}><option>10</option><option>20</option><option>50</option></select></label>
        <button type="button" class={S.footerButton}>‹</button>
        <button type="button" class={S.footerButton}>›</button>
      </div>
    </footer>
  {/if}
</section>
