<script setup lang="ts">
import { computed, ref } from "vue";
import Icon from "../../../atoms/icon/ui/Icon.vue";
import Text from "../../../atoms/text/ui/Text.vue";
import { StyleComponent } from "../../../../data/style.component";
import {
  computeDataTableView,
  getCellKind,
  normalizeActionItems,
  resolveBadgeVariantClass,
  toggleNumberInList,
} from "../../../../hooks/data-table.hook";
import DataTableFilter from "../shared/data-table-filter/ui/DataTableFilter.vue";
import type { DataTableProps, DataTableVariant, DataTextProps } from "../data/types";

const props = withDefaults(defineProps<DataTableProps>(), { variant: "01", className: "" });
const S = StyleComponent.dataTable;

const searchQuery = ref("");
const categoryFilter = ref("all");
const sortAsc = ref(true);
const selected = ref<number[]>([]);
const menuCellKey = ref<string | null>(null);
const hiddenColumns = ref<number[]>([]);
const toggleStates = ref<Record<string, boolean>>({});

const view = computed(() =>
  computeDataTableView({
    variant: props.variant as DataTableVariant,
    columns: props.columns,
    rows: props.rows,
    actions: props.actions,
    actionColumnLabel: props.actionColumnLabel,
    showActionColumn: props.showActionColumn,
    searchQuery: searchQuery.value,
    categoryFilter: categoryFilter.value,
    sortAsc: sortAsc.value,
    hiddenColumns: hiddenColumns.value,
  }),
);

function handleActionSelect(actionVal: string, ri: number, row: DataTextProps[]) {
  menuCellKey.value = null;
  props.onRowAction?.(actionVal, ri, row);
}
</script>

<template>
  <div v-if="menuCellKey" :class="S.overlay" role="presentation" @click="menuCellKey = null"></div>
  <section :class="`${S.root} ${className}`" :aria-label="title || `Data table ${variant}`">
    <h3 v-if="title" :class="S.title">{{ title }}</h3>
    <DataTableFilter
      :columns="view.columns"
      :filterOptions="view.filterOptions"
      :searchQuery="searchQuery"
      :categoryFilter="categoryFilter"
      :hiddenColumns="hiddenColumns"
      :addLabel="addLabel"
      :onAdd="onAdd"
      :onSearchChange="(v: string) => (searchQuery = v)"
      :onCategoryChange="(v: string) => (categoryFilter = v)"
      :onToggleColumn="(ci: number) => (hiddenColumns = toggleNumberInList(hiddenColumns, ci, true))"
      :onReset="() => { searchQuery = ''; categoryFilter = 'all'; hiddenColumns = []; }"
    />
    <div v-if="selected.length > 0" :class="S.selectionBar">
      <span>{{ selected.length }} selected</span>
      <button type="button" class="text-xs font-medium underline" @click="selected = []">Clear selection</button>
    </div>
    <div :class="S.viewport">
      <table :class="`${S.table} ${variant === '02' ? S.tableCompact : ''}`">
        <thead :class="S.head">
          <tr>
            <th v-if="view.showCheckbox" :class="S.checkboxCell">
              <input
                type="checkbox"
                :class="S.checkbox"
                aria-label="Select all"
                :checked="view.visibleRows.length > 0 && selected.length === view.visibleRows.length"
                @change="selected = ($event.target as HTMLInputElement).checked ? view.visibleRows.map((_, i) => i) : []"
              />
            </th>
            <template v-for="(col, ci) in view.columns" :key="ci">
              <th v-if="!hiddenColumns.includes(ci)" :class="`${S.headCell} ${col.className ?? ''}`">
                <button type="button" :class="S.headCellButton" @click="sortAsc = !sortAsc">
                  <Text :text="col.text" /><span aria-hidden="true" :class="S.headCellSortIcon">↕</span>
                </button>
              </th>
            </template>
          </tr>
        </thead>
        <tbody :class="S.tbody">
          <tr v-for="(row, ri) in view.visibleRows" :key="ri" :class="`${S.row} ${selected.includes(ri) ? S.trSelected : ''}`">
            <td v-if="view.showCheckbox" class="px-3 py-2">
              <input type="checkbox" :class="S.checkbox" :checked="selected.includes(ri)" @change="selected = toggleNumberInList(selected, ri)" />
            </td>
            <template v-for="(col, ci) in view.columns" :key="ci">
              <td v-if="!hiddenColumns.includes(ci)" :class="`${S.cell} ${variant === '02' ? S.cellCompact : ''}`">
                <a
                  v-if="getCellKind(variant as DataTableVariant, ci, row[ci], col) === 'email'"
                  :href="`mailto:${row[ci]?.text}`"
                  class="text-blue-400 underline underline-offset-2"
                >
                  <Text v-bind="row[ci] ?? { text: '' }" />
                </a>
                <span
                  v-else-if="getCellKind(variant as DataTableVariant, ci, row[ci], col) === 'badge'"
                  :class="row[ci]?.className ? row[ci]?.className : `${S.badge.base} ${resolveBadgeVariantClass(row[ci]?.badgeVariant, row[ci]?.text ?? '', S.badge)}`"
                >
                  <span v-if="row[ci]?.dot" :class="S.badge.dot" aria-hidden="true"></span>
                  <span>{{ row[ci]?.text }}</span>
                </span>
                <button
                  v-else-if="getCellKind(variant as DataTableVariant, ci, row[ci], col) === 'toggle'"
                  type="button"
                  role="switch"
                  :aria-checked="toggleStates[`${ri}-${ci}`] ?? row[ci]?.checked ?? false"
                  @click="toggleStates[`${ri}-${ci}`] = !(toggleStates[`${ri}-${ci}`] ?? row[ci]?.checked ?? false)"
                  :class="`relative inline-flex h-5 w-9 items-center rounded-full transition-colors ${(toggleStates[`${ri}-${ci}`] ?? row[ci]?.checked ?? false) ? 'bg-blue-600' : 'bg-zinc-700'}`"
                >
                  <span :class="`inline-block size-3.5 transform rounded-full bg-white transition-transform ${(toggleStates[`${ri}-${ci}`] ?? row[ci]?.checked ?? false) ? 'translate-x-4' : 'translate-x-1'}`"></span>
                </button>
                <span
                  v-else-if="getCellKind(variant as DataTableVariant, ci, row[ci], col) === 'invoice' || getCellKind(variant as DataTableVariant, ci, row[ci], col) === 'edit'"
                  class="inline-flex gap-1"
                >
                  <button type="button" :class="S.actionButton" @click="handleActionSelect(getCellKind(variant as DataTableVariant, ci, row[ci], col) === 'invoice' ? 'invoice' : 'edit', ri, row)">
                    {{ getCellKind(variant as DataTableVariant, ci, row[ci], col) === 'invoice' ? 'Invoice' : 'Edit' }}
                  </button>
                  <button type="button" :class="S.actionButton" @click="handleActionSelect(getCellKind(variant as DataTableVariant, ci, row[ci], col) === 'invoice' ? 'ledger' : 'delete', ri, row)">
                    {{ getCellKind(variant as DataTableVariant, ci, row[ci], col) === 'invoice' ? 'Ledger' : 'Delete' }}
                  </button>
                </span>
                <div
                  v-else-if="getCellKind(variant as DataTableVariant, ci, row[ci], col) === 'dropdown'"
                  class="relative inline-flex items-center gap-1"
                >
                  <button
                    type="button"
                    :class="S.menuButton"
                    aria-label="Row actions"
                    :aria-expanded="menuCellKey === `${ri}-${ci}`"
                    @click="menuCellKey = menuCellKey === `${ri}-${ci}` ? null : `${ri}-${ci}`"
                  >
                    <span v-if="row[ci]?.text" class="px-1.5 text-xs">{{ row[ci]?.text }}</span>
                    <Icon v-else name="MoreHorizontal" />
                  </button>
                  <div v-if="menuCellKey === `${ri}-${ci}`" :class="S.rowMenu" role="menu">
                    <button
                      v-for="act in normalizeActionItems(row[ci], col, actions)"
                      :key="act.label"
                      type="button"
                      role="menuitem"
                      :class="act.variant === 'danger' ? S.rowMenuDanger : S.rowMenuItem"
                      @click="handleActionSelect(act.value ?? act.label, ri, row)"
                    >
                      <Icon v-if="act.icon" :name="act.icon" />
                      <span>{{ act.label }}</span>
                    </button>
                  </div>
                </div>
                <div v-else class="flex items-center gap-2.5">
                  <img v-if="row[ci]?.avatar" :src="row[ci]?.avatar" :alt="row[ci]?.text" class="size-7 rounded-full object-cover" />
                  <div>
                    <span :class="variant === '02' && ci === 0 ? 'font-semibold text-zinc-100' : 'text-zinc-200'">
                      <Text v-bind="row[ci] ?? { text: '' }" />
                    </span>
                    <span v-if="row[ci]?.subtitle" class="block text-[11px] text-zinc-400">{{ row[ci]?.subtitle }}</span>
                  </div>
                </div>
              </td>
            </template>
          </tr>
        </tbody>
      </table>
    </div>
    <footer v-if="view.showFooter" :class="S.footer">
      <span>{{ view.pageLabel }}</span>
      <div :class="S.footerGroup">
        <label class="flex items-center gap-2">Rows per page <select :class="S.footerSelect"><option>10</option><option>20</option><option>50</option></select></label>
        <button type="button" :class="S.footerButton">‹</button>
        <button type="button" :class="S.footerButton">›</button>
      </div>
    </footer>
  </section>
</template>
