<script setup lang="ts">
import { ref } from "vue";
import Icon from "../../../../../atoms/icon/ui/Icon.vue";
import { StyleComponent } from "../../../../../../data/style.component";
import { togglePanelState, type FilterPanelType } from "../../../../../../hooks/data-table.hook";
import type { DataTableFilterProps } from "../data/types";

withDefaults(defineProps<DataTableFilterProps>(), {
  filterOptions: () => [],
  searchQuery: "",
  categoryFilter: "all",
  hiddenColumns: () => [],
});

const S = StyleComponent.dataTable;
const activePanel = ref<FilterPanelType>(null);
</script>

<template>
  <div v-if="activePanel" :class="S.overlay" role="presentation" @click="activePanel = null"></div>
  <div :class="S.toolbar">
    <label :class="S.search">
      <span :class="S.searchIcon"><Icon name="Search01" /></span>
      <input :class="S.searchInput" type="search" placeholder="Search" :value="searchQuery"
        @input="onSearchChange?.(($event.target as HTMLInputElement).value)" aria-label="Search table" />
    </label>

    <div class="ml-auto flex flex-wrap items-center gap-2">
      <label :class="S.filterSelect">
        <select :class="S.filterSelectInput" :value="categoryFilter"
          @change="onCategoryChange?.(($event.target as HTMLSelectElement).value)" aria-label="Category filter">
          <option value="all">All</option>
          <option v-for="opt in filterOptions" :key="opt" :value="opt">{{ opt }}</option>
        </select>
        <span :class="S.filterSelectIcon"><Icon name="ArrowDown01" /></span>
      </label>

      <div class="relative">
        <button type="button" :class="`${S.button} ${activePanel === 'filter' ? S.buttonActive : ''}`"
          @click="activePanel = togglePanelState(activePanel, 'filter')">
          <Icon name="FilterHorizontal" /><span>Filter</span>
        </button>
        <div v-if="activePanel === 'filter'" :class="S.panelLeft" role="menu">
          <button type="button" :class="S.option" @click="onCategoryChange?.('all'); activePanel = null"><span :class="S.optionLabel">All</span></button>
          <button v-for="opt in filterOptions" :key="opt" type="button" :class="S.option"
            @click="onCategoryChange?.(opt); activePanel = null"><span :class="S.optionLabel">{{ opt }}</span></button>
        </div>
      </div>

      <div class="relative">
        <button type="button" :class="`${S.button} ${activePanel === 'columns' ? S.buttonActive : ''}`"
          @click="activePanel = togglePanelState(activePanel, 'columns')">
          <Icon name="LayoutThreeColumn" /><span>Columns</span>
        </button>
        <div v-if="activePanel === 'columns'" :class="S.panelRight" role="menu">
          <button v-for="(col, ci) in columns" :key="ci" type="button"
            :class="`${S.option} ${ci === 0 ? S.optionDisabled : ''}`" :disabled="ci === 0"
            @click="onToggleColumn?.(ci)" role="menuitemcheckbox" :aria-checked="!hiddenColumns?.includes(ci)">
            <input type="checkbox" :class="S.optionInput" :checked="!hiddenColumns?.includes(ci)" :disabled="ci === 0" readonly />
            <span :class="S.optionLabel">{{ col.text }}</span>
            <span v-if="ci !== 0" :class="S.dragHandle"><Icon name="DragDropVertical" /></span>
          </button>
        </div>
      </div>

      <div class="relative">
        <button type="button" :class="S.iconButton" aria-label="More options"
          @click="activePanel = togglePanelState(activePanel, 'more')"><Icon name="MoreVertical" /></button>
        <div v-if="activePanel === 'more'" :class="S.menu" role="menu">
          <button type="button" :class="S.menuItem" @click="onReset?.(); activePanel = null">Reset filters</button>
          <button type="button" :class="S.menuItem" @click="activePanel = null">Export CSV</button>
        </div>
      </div>

      <button v-if="addLabel || onAdd" type="button" :class="S.primaryButton" @click="onAdd?.()">
        <span>+ {{ addLabel ?? 'Add new' }}</span>
      </button>
    </div>
  </div>
</template>
