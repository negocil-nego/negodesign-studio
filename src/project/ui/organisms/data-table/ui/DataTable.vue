<script setup lang="ts">
import { computed, ref } from "vue";
import Icon from "../../../atoms/icon/ui/Icon.vue";
import Text from "../../../atoms/text/ui/Text.vue";
import { StyleComponent } from "../../../../data/style.component";
import { computeDataTableView, DATA_TABLE_ROW_ACTIONS, getCellKind, toggleNumberInList } from "../../../../hooks/data-table.hook";
import DataTableFilter from "../shared/data-table-filter/ui/DataTableFilter.vue";
import type { DataTableProps, DataTableVariant } from "../data/types";

const props = withDefaults(defineProps<DataTableProps>(), { variant: "01", className: "" });
const S = StyleComponent.dataTable;

const searchQuery = ref("");
const categoryFilter = ref("all");
const sortAsc = ref(true);
const selected = ref<number[]>([]);
const menuRow = ref<number | null>(null);
const hiddenColumns = ref<number[]>([]);

const view = computed(() =>
  computeDataTableView({
    variant: props.variant as DataTableVariant,
    columns: props.columns,
    rows: props.rows,
    searchQuery: searchQuery.value,
    categoryFilter: categoryFilter.value,
    sortAsc: sortAsc.value,
    hiddenColumns: hiddenColumns.value,
  }),
);
</script>

<template>
  <section :class="`${S.root} ${className}`" :aria-label="title || `Data table ${variant}`">
    <h3 v-if="title" :class="S.title">{{ title }}</h3>
    <DataTableFilter
      :columns="view.columns"
      :filterOptions="view.filterOptions"
      :searchQuery="searchQuery"
      :categoryFilter="categoryFilter"
      :hiddenColumns="hiddenColumns"
      :onSearchChange="(v: string) => (searchQuery = v)"
      :onCategoryChange="(v: string) => (categoryFilter = v)"
      :onToggleColumn="(ci: number) => (hiddenColumns = toggleNumberInList(hiddenColumns, ci, true))"
      :onReset="() => { searchQuery = ''; categoryFilter = 'all'; hiddenColumns = []; }"
    />
    <div :class="S.viewport">
      <table :class="`${S.table} ${variant === '02' ? S.tableCompact : ''}`">
        <thead :class="S.head">
          <tr>
            <th v-if="view.showCheckbox" :class="S.checkboxCell"><input type="checkbox" :class="S.checkbox" aria-label="Select all" /></th>
            <template v-for="(col, ci) in view.columns" :key="ci">
              <th v-if="!hiddenColumns.includes(ci)" :class="`${S.headCell} ${col.className ?? ''}`">
                <button :class="S.headCellButton" @click="sortAsc = !sortAsc">
                  <Text :text="col.text" /><span aria-hidden="true" :class="S.headCellSortIcon">↕</span>
                </button>
              </th>
            </template>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(row, ri) in view.visibleRows" :key="ri" :class="S.row">
            <td v-if="view.showCheckbox" class="px-3 py-2">
              <input type="checkbox" :class="S.checkbox" :checked="selected.includes(ri)" @change="selected = toggleNumberInList(selected, ri)" />
            </td>
            <template v-for="(col, ci) in view.columns" :key="ci">
              <td v-if="!hiddenColumns.includes(ci)" :class="`${S.cell} ${variant === '02' ? S.cellCompact : ''}`">
                <a v-if="getCellKind(variant as DataTableVariant, ci) === 'email'" :href="`mailto:${row[ci]?.text}`" class="text-blue-400 underline underline-offset-2"><Text v-bind="row[ci] ?? { text: '' }" /></a>
                <span v-else-if="getCellKind(variant as DataTableVariant, ci) === 'invoice' || getCellKind(variant as DataTableVariant, ci) === 'edit'" class="inline-flex gap-1">
                  <button :class="S.actionButton">{{ getCellKind(variant as DataTableVariant, ci) === 'invoice' ? 'Invoice' : 'Edit' }}</button>
                  <button :class="S.actionButton">{{ getCellKind(variant as DataTableVariant, ci) === 'invoice' ? 'Ledger' : 'Delete' }}</button>
                </span>
                <span v-else-if="getCellKind(variant as DataTableVariant, ci) === 'menu'" class="relative inline-flex gap-1">
                  <button :class="S.actionButton"><Icon name="Phone" /></button>
                  <button :class="S.actionButton"><Icon name="FolderAdd" /></button>
                  <button :class="S.actionButton" @click="menuRow = menuRow === ri ? null : ri"><Icon name="MoreHorizontal" /></button>
                  <span v-if="menuRow === ri" :class="S.rowMenu"><button v-for="a in DATA_TABLE_ROW_ACTIONS" :key="a" :class="S.rowMenuItem">{{ a }}</button></span>
                </span>
                <span v-else :class="variant === '02' && ci === 0 ? 'font-semibold text-zinc-100' : 'text-zinc-200'"><Text v-bind="row[ci] ?? { text: '' }" /></span>
              </td>
            </template>
          </tr>
        </tbody>
      </table>
    </div>
    <footer v-if="view.showFooter" :class="S.footer">
      <span>{{ view.pageLabel }}</span>
      <div :class="S.footerGroup">
        <label class="flex items-center gap-2">Rows per page <select :class="S.footerSelect"><option>10</option><option>20</option></select></label>
        <button :class="S.footerButton">‹</button><button :class="S.footerButton">›</button>
      </div>
    </footer>
  </section>
</template>
