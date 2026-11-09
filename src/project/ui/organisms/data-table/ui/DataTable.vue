<script setup lang="ts">
import { computed, ref } from "vue";
import Icon from "../../../atoms/icon/ui/Icon.vue";
import Text from "../../../atoms/text/ui/Text.vue";
import { StyleComponent } from "../../../../data/style.component";
import { dataTableExampleByVariant } from "../data/examples";
import DataTableFilter from "../shared/data-table-filter/ui/DataTableFilter.vue";
import type { DataTableProps, DataTableVariant } from "../data/types";
import type { TextProps } from "../../../atoms/text/data/types";

const props = withDefaults(defineProps<DataTableProps>(), { variant: "01", className: "" });

const S = StyleComponent.dataTable;

const searchQuery = ref("");
const categoryFilter = ref("all");
const sortAsc = ref(true);
const selected = ref<number[]>([]);
const menuRow = ref<number | null>(null);
const hiddenColumns = ref<number[]>([]);

const example = computed(() => dataTableExampleByVariant[props.variant as DataTableVariant] ?? dataTableExampleByVariant["01"]);
const cols = computed<TextProps[]>(() => props.columns ?? example.value.columns);

const allChunkedRows = computed<TextProps[][]>(() => {
  const flat = props.rows ?? example.value.rows;
  const n = cols.value.length || 1;
  const result: TextProps[][] = [];
  for (let i = 0; i < flat.length; i += n) result.push(flat.slice(i, i + n));
  return result;
});

const filterOptions = computed<string[]>(() => {
  const colIdx = cols.value.length > 1 ? 1 : 0;
  const values = new Set<string>();
  for (const row of allChunkedRows.value) {
    const val = row[colIdx]?.text?.trim();
    if (val) values.add(val);
  }
  return [...values];
});

const visibleRows = computed<TextProps[][]>(() => {
  let list = sortAsc.value ? [...allChunkedRows.value] : [...allChunkedRows.value].reverse();
  if (categoryFilter.value !== "all") {
    const colIdx = cols.value.length > 1 ? 1 : 0;
    list = list.filter((row) => row[colIdx]?.text === categoryFilter.value);
  }
  if (searchQuery.value.trim()) {
    const q = searchQuery.value.toLowerCase();
    list = list.filter((row) => row.some((c) => c.text.toLowerCase().includes(q)));
  }
  return list;
});

function toggleCol(i: number) {
  if (i === 0) return;
  hiddenColumns.value = hiddenColumns.value.includes(i)
    ? hiddenColumns.value.filter((x) => x !== i)
    : [...hiddenColumns.value, i];
}

function toggleSelected(i: number) {
  selected.value = selected.value.includes(i)
    ? selected.value.filter((x) => x !== i)
    : [...selected.value, i];
}

function resetFilters() {
  searchQuery.value = "";
  categoryFilter.value = "all";
  hiddenColumns.value = [];
}
</script>

<template>
  <section :class="`${S.root} ${className}`" :aria-label="title || `Tabela de dados, variação ${variant}`">
    <h3 v-if="title" :class="S.title">{{ title }}</h3>

    <DataTableFilter
      :columns="cols"
      :filterOptions="filterOptions"
      :searchQuery="searchQuery"
      :categoryFilter="categoryFilter"
      :hiddenColumns="hiddenColumns"
      :onSearchChange="(val: string) => (searchQuery = val)"
      :onCategoryChange="(val: string) => (categoryFilter = val)"
      :onToggleColumn="toggleCol"
      :onReset="resetFilters"
    />

    <div :class="S.viewport">
      <table :class="`${S.table} ${variant === '02' ? S.tableCompact : ''}`">
        <thead :class="S.head">
          <tr>
            <th v-if="variant !== '01'" :class="S.checkboxCell">
              <input type="checkbox" :class="S.checkbox" aria-label="Seleccionar todas" />
            </th>
            <template v-for="(col, ci) in cols" :key="ci">
              <th v-if="!hiddenColumns.includes(ci)" :class="`${S.headCell} ${col.className ?? ''}`">
                <button :class="S.headCellButton" @click="sortAsc = !sortAsc" :aria-label="`Ordenar por ${col.text}`">
                  <Text :text="col.text" />
                  <span aria-hidden="true" :class="S.headCellSortIcon">↕</span>
                </button>
              </th>
            </template>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(row, ri) in visibleRows" :key="ri" :class="S.row">
            <td v-if="variant !== '01'" class="px-3 py-2">
              <input type="checkbox" :class="S.checkbox" :checked="selected.includes(ri)" @change="toggleSelected(ri)" />
            </td>
            <template v-for="(col, ci) in cols" :key="ci">
              <td v-if="!hiddenColumns.includes(ci)" :class="`${S.cell} ${variant === '02' ? S.cellCompact : ''}`">
                <a v-if="variant === '03' && ci === 1" :href="`mailto:${row[ci]?.text}`" class="text-blue-400 underline underline-offset-2">
                  <Text v-bind="row[ci] ?? { text: '' }" />
                </a>
                <span v-else-if="variant === '04' && ci === 7" class="inline-flex gap-1">
                  <button :class="S.actionButton">Invoice</button>
                  <button :class="S.actionButton">Ledger</button>
                </span>
                <span v-else-if="variant === '01' && ci === 6" class="relative inline-flex gap-1">
                  <button :class="S.actionButton" aria-label="Ligar"><Icon name="Phone" /></button>
                  <button :class="S.actionButton" aria-label="Adicionar"><Icon name="FolderAdd" /></button>
                  <button :class="S.actionButton" @click="menuRow = menuRow === ri ? null : ri"><Icon name="MoreHorizontal" /></button>
                  <span v-if="menuRow === ri" :class="S.rowMenu">
                    <button v-for="a in ['Details', 'Offer a deal', 'Add to my list', 'Delete']" :key="a" :class="S.rowMenuItem">{{ a }}</button>
                  </span>
                </span>
                <span v-else-if="variant === '03' && ci === 6" class="inline-flex gap-1">
                  <button :class="S.actionButton">Edit</button>
                  <button :class="S.actionButton">Delete</button>
                </span>
                <span v-else :class="variant === '02' && ci === 0 ? 'font-semibold text-zinc-100' : 'text-zinc-200'">
                  <Text v-bind="row[ci] ?? { text: '' }" />
                </span>
              </td>
            </template>
          </tr>
        </tbody>
      </table>
    </div>

    <footer v-if="variant !== '01' && variant !== '05'" :class="S.footer">
      <span>{{ variant === "02" ? "Page 1 of 13" : "Page 1 of 8" }}</span>
      <div :class="S.footerGroup">
        <label class="flex items-center gap-2">
          Rows per page
          <select :class="S.footerSelect"><option>10</option><option>20</option></select>
        </label>
        <button :class="S.footerButton">‹</button>
        <button :class="S.footerButton">›</button>
      </div>
    </footer>
  </section>
</template>
