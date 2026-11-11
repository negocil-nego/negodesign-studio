<script lang="ts">
  import Icon from "../../../../../atoms/icon/ui/Icon.svelte";
  import { StyleComponent } from "../../../../../../data/style.component";
  import { togglePanelState, type FilterPanelType } from "../../../../../../hooks/data-table.hook";
  import type { DataTableFilterProps } from "../data/types";

  let {
    columns = [],
    filterOptions = [],
    searchQuery = "",
    categoryFilter = "all",
    hiddenColumns = [],
    addLabel,
    onSearchChange,
    onCategoryChange,
    onToggleColumn,
    onReset,
    onAdd,
  }: DataTableFilterProps = $props();

  const S = StyleComponent.dataTable;
  let activePanel = $state<FilterPanelType>(null);
  const selectCategory = (val: string) => { onCategoryChange?.(val); activePanel = null; };
</script>

{#if activePanel}
  <div class={S.overlay} role="presentation" onclick={() => (activePanel = null)}></div>
{/if}

<div class={S.toolbar}>
  <label class={S.search}>
    <span class={S.searchIcon}><Icon name="Search01" /></span>
    <input class={S.searchInput} type="search" placeholder="Search" value={searchQuery}
      oninput={(e) => onSearchChange?.((e.currentTarget as HTMLInputElement).value)} aria-label="Search table" />
  </label>

  <div class="ml-auto flex flex-wrap items-center gap-2">
    <label class={S.filterSelect}>
      <select class={S.filterSelectInput} value={categoryFilter}
        onchange={(e) => onCategoryChange?.((e.currentTarget as HTMLSelectElement).value)} aria-label="Category filter">
        <option value="all">All</option>
        {#each filterOptions as opt (opt)}<option value={opt}>{opt}</option>{/each}
      </select>
      <span class={S.filterSelectIcon}><Icon name="ArrowDown01" /></span>
    </label>

    <div class="relative">
      <button type="button" class={`${S.button} ${activePanel === "filter" ? S.buttonActive : ""}`}
        onclick={() => (activePanel = togglePanelState(activePanel, "filter"))}>
        <Icon name="FilterHorizontal" /><span>Filter</span>
      </button>
      {#if activePanel === "filter"}
        <div class={S.panelLeft} role="menu">
          <button type="button" class={S.option} onclick={() => selectCategory("all")}><span class={S.optionLabel}>All</span></button>
          {#each filterOptions as opt (opt)}
            <button type="button" class={S.option} onclick={() => selectCategory(opt)}><span class={S.optionLabel}>{opt}</span></button>
          {/each}
        </div>
      {/if}
    </div>

    <div class="relative">
      <button type="button" class={`${S.button} ${activePanel === "columns" ? S.buttonActive : ""}`}
        onclick={() => (activePanel = togglePanelState(activePanel, "columns"))}>
        <Icon name="LayoutThreeColumn" /><span>Columns</span>
      </button>
      {#if activePanel === "columns"}
        <div class={S.panelRight} role="menu">
          {#each columns as col, ci (ci)}
            <button type="button" class={`${S.option} ${ci === 0 ? S.optionDisabled : ""}`} disabled={ci === 0}
              onclick={() => onToggleColumn?.(ci)} role="menuitemcheckbox" aria-checked={!hiddenColumns.includes(ci)}>
              <input type="checkbox" class={S.optionInput} checked={!hiddenColumns.includes(ci)} disabled={ci === 0} readonly />
              <span class={S.optionLabel}>{col.text}</span>
              {#if ci !== 0}<span class={S.dragHandle}><Icon name="DragDropVertical" /></span>{/if}
            </button>
          {/each}
        </div>
      {/if}
    </div>

    <div class="relative">
      <button type="button" class={S.iconButton} aria-label="More options"
        onclick={() => (activePanel = togglePanelState(activePanel, "more"))}><Icon name="MoreVertical" /></button>
      {#if activePanel === "more"}
        <div class={S.menu} role="menu">
          <button type="button" class={S.menuItem} onclick={() => { onReset?.(); activePanel = null; }}>Reset filters</button>
          <button type="button" class={S.menuItem} onclick={() => (activePanel = null)}>Export CSV</button>
        </div>
      {/if}
    </div>

    {#if addLabel || onAdd}
      <button type="button" class={S.primaryButton} onclick={() => onAdd?.()}>
        <span>+ {addLabel ?? "Add new"}</span>
      </button>
    {/if}
  </div>
</div>
