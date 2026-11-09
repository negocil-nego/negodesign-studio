<script lang="ts">
  import Icon from "../../../atoms/icon/ui/Icon.svelte";
  import Text from "../../../atoms/text/ui/Text.svelte";
  import { StyleComponent } from "../../../data/style.component";
  import { dataTableExampleByVariant } from "../data/examples";
  import type { DataTableProps, DataTableVariant } from "../data/types";
  import type { TextProps } from "../../../atoms/text/data/types";

  const S = StyleComponent.dataTable;
  const ALL = "__all__";

  let {
    variant = "01",
    title,
    columns: suppliedColumns,
    rows: suppliedRows,
    className = "",
    searchPlaceholder = "Search",
    filterColumn = 0,
    filterAllLabel,
    showToolbar = true,
    moreActions = ["Export CSV", "Print"],
  }: DataTableProps = $props();

  let sortAsc = $state(true);
  let selected = $state<number[]>([]);
  let menuRowIndex = $state<number | null>(null);
  let query = $state("");
  let filterValue = $state<string>(ALL);
  let filterValues = $state<string[]>([]);
  let hidden = $state<number[]>([]);
  let order = $state<number[] | null>(null);
  let openPanel = $state<"filter" | "columns" | "more" | null>(null);
  let dragFrom: number | null = null;

  const example = $derived(
    dataTableExampleByVariant[variant as DataTableVariant] ?? dataTableExampleByVariant["01"],
  );

  const columns = $derived<TextProps[]>(suppliedColumns ?? example.columns);

  /** Chunk flat rows array into 2-D by columns.length. */
  const allRows = $derived.by(() => {
    const flat = suppliedRows ?? example.rows;
    const n = columns.length || 1;
    const result: TextProps[][] = [];
    for (let i = 0; i < flat.length; i += n) result.push(flat.slice(i, i + n));
    return result;
  });

  const filterOptions = $derived.by(() => {
    const seen: string[] = [];
    for (const row of allRows) {
      const value = row[filterColumn]?.text ?? "";
      if (value && !seen.includes(value)) seen.push(value);
    }
    return seen;
  });

  const rows = $derived.by(() => {
    const q = query.trim().toLowerCase();
    const filtered = allRows.filter((row) => {
      const value = row[filterColumn]?.text ?? "";
      if (filterValue !== ALL && value !== filterValue) return false;
      if (filterValues.length > 0 && !filterValues.includes(value)) return false;
      if (q && !row.some((cell) => cell.text.toLowerCase().includes(q))) return false;
      return true;
    });
    return sortAsc ? filtered : [...filtered].reverse();
  });

  const columnOrder = $derived(
    order && order.length === columns.length ? order : columns.map((_, i) => i),
  );
  const visibleCols = $derived(columnOrder.filter((i) => !hidden.includes(i)));
  const filterLabel = $derived(filterAllLabel ?? `All ${columns[filterColumn]?.text ?? ""}`);

  function toggleSelected(i: number) {
    selected = selected.includes(i) ? selected.filter((x) => x !== i) : [...selected, i];
  }

  function toggleColumn(index: number) {
    if (index === 0) return;
    hidden = hidden.includes(index) ? hidden.filter((x) => x !== index) : [...hidden, index];
  }

  function toggleFilterValue(value: string) {
    filterValues = filterValues.includes(value) ? filterValues.filter((x) => x !== value) : [...filterValues, value];
  }

  function moveColumn(from: number, to: number) {
    const next = [...columnOrder];
    const [moved] = next.splice(from, 1);
    next.splice(to, 0, moved);
    order = next;
  }

  function dropColumn(pos: number) {
    if (dragFrom !== null && dragFrom !== pos) moveColumn(dragFrom, pos);
    dragFrom = null;
  }
</script>

<section
  class={`${S.root} ${className}`}
  aria-label={title ?? `Tabela de dados, variação ${variant}`}
>
  {#if openPanel}<div class={S.overlay} onclick={() => (openPanel = null)}></div>{/if}
  {#if title}<h3 class={S.title}>{title}</h3>{/if}

  {#if showToolbar}
    <div class={S.toolbar}>
      <div class={S.search}>
        <Icon name="search-01" className={S.searchIcon} />
        <input
          type="search"
          class={S.searchInput}
          placeholder={searchPlaceholder}
          aria-label={searchPlaceholder}
          bind:value={query}
        />
      </div>

      <div class={S.filterSelect}>
        <select class={S.filterSelectInput} aria-label={filterLabel} bind:value={filterValue}>
          <option value={ALL}>{filterLabel}</option>
          {#each filterOptions as value (value)}
            <option {value}>{value}</option>
          {/each}
        </select>
        <Icon name="chevron-down" className={S.filterSelectIcon} />
      </div>

      <div class="relative">
        <button
          type="button"
          class={`${S.button} ${openPanel === "filter" ? S.buttonActive : ""}`}
          aria-label="Filtrar linhas"
          aria-expanded={openPanel === "filter"}
          onclick={() => (openPanel = openPanel === "filter" ? null : "filter")}
        >
          <Icon name="filter" />
          Filter
        </button>
        {#if openPanel === "filter"}
          <div class={S.panelLeft}>
            {#each filterOptions as value (value)}
              <label class={S.option}>
                <input
                  type="checkbox"
                  class={S.optionInput}
                  checked={filterValues.includes(value)}
                  onchange={() => toggleFilterValue(value)}
                />
                <span class={S.optionLabel}>{value}</span>
              </label>
            {/each}
          </div>
        {/if}
      </div>

      <div class="relative">
        <button
          type="button"
          class={`${S.button} ${openPanel === "columns" ? S.buttonActive : ""}`}
          aria-label="Gerir colunas"
          aria-expanded={openPanel === "columns"}
          onclick={() => (openPanel = openPanel === "columns" ? null : "columns")}
        >
          <Icon name="layout-three-column" />
          Columns
        </button>
        {#if openPanel === "columns"}
          <div class={S.panelRight}>
            {#each columnOrder as index, pos (index)}
              <div
                class={`${S.option} ${index === 0 ? S.optionDisabled : "cursor-grab"}`}
                draggable={index !== 0}
                ondragstart={() => (dragFrom = pos)}
                ondragover={(e) => e.preventDefault()}
                ondrop={() => dropColumn(pos)}
              >
                <input
                  type="checkbox"
                  class={S.optionInput}
                  checked={!hidden.includes(index)}
                  disabled={index === 0}
                  aria-label={`Mostrar ${columns[index]?.text ?? ""}`}
                  onchange={() => toggleColumn(index)}
                />
                <span class={S.optionLabel}>{columns[index]?.text}</span>
                <span class={S.dragHandle} aria-hidden="true"><Icon name="grip-vertical" /></span>
              </div>
            {/each}
          </div>
        {/if}
      </div>

      <div class="relative">
        <button
          type="button"
          class={S.iconButton}
          aria-label="Mais acções"
          aria-expanded={openPanel === "more"}
          onclick={() => (openPanel = openPanel === "more" ? null : "more")}
        >
          <Icon name="more-vertical" />
        </button>
        {#if openPanel === "more"}
          <div class={S.menu}>
            {#each moreActions as action (action)}
              <button type="button" class={S.menuItem} onclick={() => (openPanel = null)}>{action}</button>
            {/each}
          </div>
        {/if}
      </div>
    </div>
  {/if}

  <div class={S.viewport}>
    <table class={`${S.table} ${variant === "02" ? S.tableCompact : ""}`}>

      <!-- Head -->
      <thead class={S.head}>
        <tr>
          {#if variant !== "01"}
            <th class={S.checkboxCell}><input type="checkbox" class={S.checkbox} aria-label="Seleccionar todas as linhas" /></th>
          {/if}
          {#each visibleCols as index (index)}
            <th class={`${S.headCell} ${columns[index]?.className ?? ""}`}>
              <button
                type="button"
                class={S.headCellButton}
                onclick={() => (sortAsc = !sortAsc)}
                aria-label={`Ordenar por ${columns[index]?.text}`}
              >
                <Text text={columns[index]?.text ?? ""} />
                <span aria-hidden="true" class={S.headCellSortIcon}>↕</span>
              </button>
            </th>
          {/each}
        </tr>
      </thead>

      <!-- Body -->
      <tbody>
        {#each rows as row, ri (ri)}
          <tr class={S.row}>
            {#if variant !== "01"}
              <td class={S.checkboxCell}>
                <input
                  type="checkbox"
                  class={S.checkbox}
                  checked={selected.includes(ri)}
                  aria-label={`Seleccionar ${row[0]?.text ?? ""}`}
                  onchange={() => toggleSelected(ri)}
                />
              </td>
            {/if}

            {#each visibleCols as index (index)}
              {@const cell = row[index] ?? { text: "" }}
              <td class={`${S.cell} ${variant === "02" ? S.cellCompact : ""}`}>

                {#if variant === "03" && index === 1}
                  <!-- Email link -->
                  <a href={`mailto:${cell.text}`} class="text-slate-600 underline underline-offset-2">
                    <Text {...cell} />
                  </a>

                {:else if variant === "04" && index === 7}
                  <!-- Invoice/Ledger actions -->
                  <span class="inline-flex gap-1">
                    <button type="button" class={S.actionButton}>Invoice</button>
                    <button type="button" class={S.actionButton}>Ledger</button>
                  </span>

                {:else if variant === "01" && index === 6}
                  <!-- Row action menu -->
                  <span class="relative inline-flex gap-1">
                    <button type="button" class={S.actionButton} aria-label="Ligar"><Icon name="Phone" /></button>
                    <button type="button" class={S.actionButton} aria-label="Adicionar"><Icon name="FolderAdd" /></button>
                    <button
                      type="button"
                      class={S.actionButton}
                      aria-label="Mais acções"
                      onclick={() => (menuRowIndex = menuRowIndex === ri ? null : ri)}
                    ><Icon name="MoreHorizontal" /></button>
                    {#if menuRowIndex === ri}
                      <span class={S.rowMenu}>
                        {#each ["Details", "Offer a deal", "Add to my list", "Delete"] as action (action)}
                          <button type="button" class={S.rowMenuItem}>{action}</button>
                        {/each}
                      </span>
                    {/if}
                  </span>

                {:else if variant === "03" && index === 6}
                  <!-- Edit/Delete actions -->
                  <span class="inline-flex gap-1">
                    <button type="button" class={S.actionButton}>Edit</button>
                    <button type="button" class={S.actionButton}>Delete</button>
                  </span>

                {:else}
                  <!-- Generic: Text atom handles className (badges etc.) -->
                  <span class={variant === "02" && index === 0 ? "font-semibold" : ""}>
                    <Text {...cell} />
                  </span>
                {/if}

              </td>
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
        <label>Rows per page
          <select class={S.footerSelect} aria-label="Linhas por página">
            <option>10</option><option>20</option>
          </select>
        </label>
        <button type="button" class={S.footerButton} aria-label="Página anterior">‹</button>
        <button type="button" class={S.footerButton} aria-label="Página seguinte">›</button>
      </div>
    </footer>
  {/if}
</section>
