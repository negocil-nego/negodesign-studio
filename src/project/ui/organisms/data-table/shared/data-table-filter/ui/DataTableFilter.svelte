<script lang="ts">
  import Icon from "../../../../../atoms/icon/ui/Icon.svelte";
  import { StyleComponent } from "../../../../../../data/style.component";
  import type { DataTableFilterProps } from "../data/types";

  let {
    columns = [],
    filterOptions = [],
    searchQuery = "",
    categoryFilter = "all",
    hiddenColumns = [],
    onSearchChange,
    onCategoryChange,
    onToggleColumn,
    onReset,
  }: DataTableFilterProps = $props();

  const S = StyleComponent.dataTable;

  let activePanel = $state<"filter" | "columns" | "more" | null>(null);

  function isHidden(index: number) {
    return hiddenColumns.includes(index);
  }
</script>

{#if activePanel !== null}
  <div
    class={S.overlay}
    role="presentation"
    onclick={() => (activePanel = null)}
  ></div>
{/if}

<div class={S.toolbar}>
  <!-- Search -->
  <label class={S.search}>
    <span class={S.searchIcon}><Icon name="Search01" /></span>
    <input
      class={S.searchInput}
      type="search"
      placeholder="Search"
      value={searchQuery}
      oninput={(e) => onSearchChange?.((e.currentTarget as HTMLInputElement).value)}
      aria-label="Pesquisar na tabela"
    />
  </label>

  <div class="ml-auto flex flex-wrap items-center gap-2">
    <!-- Category / Campaign select -->
    <label class={S.filterSelect}>
      <select
        class={S.filterSelectInput}
        value={categoryFilter}
        onchange={(e) => onCategoryChange?.((e.currentTarget as HTMLSelectElement).value)}
        aria-label="Filtrar por categoria"
      >
        <option value="all">All Campaigns</option>
        {#each filterOptions as opt (opt)}
          <option value={opt}>{opt}</option>
        {/each}
      </select>
      <span class={S.filterSelectIcon}><Icon name="ArrowDown01" /></span>
    </label>

    <!-- Filter button -->
    <div class="relative">
      <button
        type="button"
        class={`${S.button} ${activePanel === "filter" ? S.buttonActive : ""}`}
        onclick={() => (activePanel = activePanel === "filter" ? null : "filter")}
        aria-expanded={activePanel === "filter"}
      >
        <Icon name="FilterHorizontal" />
        <span>Filter</span>
      </button>

      {#if activePanel === "filter"}
        <div class={S.panelLeft} role="menu">
          <button
            type="button"
            class={S.option}
            onclick={() => { onCategoryChange?.("all"); activePanel = null; }}
          >
            <span class={S.optionLabel}>All Campaigns</span>
          </button>
          {#each filterOptions as opt (opt)}
            <button
              type="button"
              class={S.option}
              onclick={() => { onCategoryChange?.(opt); activePanel = null; }}
            >
              <span class={S.optionLabel}>{opt}</span>
            </button>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Columns button + dropdown -->
    <div class="relative">
      <button
        type="button"
        class={`${S.button} ${activePanel === "columns" ? S.buttonActive : ""}`}
        onclick={() => (activePanel = activePanel === "columns" ? null : "columns")}
        aria-expanded={activePanel === "columns"}
      >
        <Icon name="LayoutThreeColumn" />
        <span>Columns</span>
      </button>

      {#if activePanel === "columns"}
        <div class={S.panelRight} role="menu">
          {#each columns as col, ci (ci)}
            <button
              type="button"
              class={`${S.option} ${ci === 0 ? S.optionDisabled : ""}`}
              disabled={ci === 0}
              onclick={() => ci !== 0 && onToggleColumn?.(ci)}
              role="menuitemcheckbox"
              aria-checked={!isHidden(ci)}
            >
              <input
                type="checkbox"
                class={S.optionInput}
                checked={!isHidden(ci)}
                disabled={ci === 0}
                readonly
              />
              <span class={S.optionLabel}>{col.text}</span>
              {#if ci !== 0}
                <span class={S.dragHandle}><Icon name="DragDropVertical" /></span>
              {/if}
            </button>
          {/each}
        </div>
      {/if}
    </div>

    <!-- More actions -->
    <div class="relative">
      <button
        type="button"
        class={S.iconButton}
        aria-label="Mais opções"
        onclick={() => (activePanel = activePanel === "more" ? null : "more")}
      >
        <Icon name="MoreVertical" />
      </button>

      {#if activePanel === "more"}
        <div class={S.menu} role="menu">
          <button
            type="button"
            class={S.menuItem}
            onclick={() => { onReset?.(); activePanel = null; }}
          >
            Reset filters
          </button>
          <button type="button" class={S.menuItem} onclick={() => (activePanel = null)}>
            Export CSV
          </button>
        </div>
      {/if}
    </div>
  </div>
</div>
