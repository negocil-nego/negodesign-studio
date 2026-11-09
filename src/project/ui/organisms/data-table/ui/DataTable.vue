<script setup lang="ts">
import { computed, ref } from "vue";
import Icon from "../../../atoms/icon/ui/Icon.vue";
import Text from "../../../atoms/text/ui/Text.vue";
import { StyleComponent } from "../../../data/style.component";
import { dataTableExampleByVariant } from "../data/examples";
import type { DataTableProps, DataTableVariant } from "../data/types";
import type { TextProps } from "../../../atoms/text/data/types";

const S = StyleComponent.dataTable;
const ALL = "__all__";

const props = withDefaults(defineProps<DataTableProps>(), {
  variant: "01",
  className: "",
  searchPlaceholder: "Search",
  filterColumn: 0,
  showToolbar: true,
  moreActions: () => ["Export CSV", "Print"],
});

const sortAsc = ref(true);
const selected = ref<number[]>([]);
const menuRow = ref<number | null>(null);
const query = ref("");
const filterValue = ref<string>(ALL);
const filterValues = ref<string[]>([]);
const hidden = ref<number[]>([]);
const order = ref<number[] | null>(null);
const open = ref<"filter" | "columns" | "more" | null>(null);
let dragFrom: number | null = null;

const example = computed(() => dataTableExampleByVariant[props.variant as DataTableVariant] ?? dataTableExampleByVariant["01"]);
const cols = computed<TextProps[]>(() => props.columns ?? example.value.columns);

const allRows = computed<TextProps[][]>(() => {
  const flat = props.rows ?? example.value.rows;
  const n = cols.value.length || 1;
  const result: TextProps[][] = [];
  for (let i = 0; i < flat.length; i += n) result.push(flat.slice(i, i + n));
  return result;
});

const filterOptions = computed<string[]>(() => {
  const seen: string[] = [];
  for (const row of allRows.value) {
    const value = row[props.filterColumn]?.text ?? "";
    if (value && !seen.includes(value)) seen.push(value);
  }
  return seen;
});

const chunkedRows = computed<TextProps[][]>(() => {
  const q = query.value.trim().toLowerCase();
  const filtered = allRows.value.filter((row) => {
    const value = row[props.filterColumn]?.text ?? "";
    if (filterValue.value !== ALL && value !== filterValue.value) return false;
    if (filterValues.value.length > 0 && !filterValues.value.includes(value)) return false;
    if (q && !row.some((cell) => cell.text.toLowerCase().includes(q))) return false;
    return true;
  });
  return sortAsc.value ? filtered : [...filtered].reverse();
});

const currentOrder = computed<number[]>(
  () => order.value && order.value.length === cols.value.length ? order.value : cols.value.map((_, i) => i),
);
const visibleCols = computed<number[]>(() => currentOrder.value.filter((i) => !hidden.value.includes(i)));
const filterLabel = computed(() => props.filterAllLabel ?? `All ${cols.value[props.filterColumn]?.text ?? ""}`);

function toggle(i: number) {
  selected.value = selected.value.includes(i)
    ? selected.value.filter((x) => x !== i)
    : [...selected.value, i];
}

function toggleColumn(index: number) {
  if (index === 0) return;
  hidden.value = hidden.value.includes(index)
    ? hidden.value.filter((x) => x !== index)
    : [...hidden.value, index];
}

function toggleFilterValue(value: string) {
  filterValues.value = filterValues.value.includes(value)
    ? filterValues.value.filter((x) => x !== value)
    : [...filterValues.value, value];
}

function moveColumn(from: number, to: number) {
  const next = [...currentOrder.value];
  const [moved] = next.splice(from, 1);
  next.splice(to, 0, moved);
  order.value = next;
}

function dropColumn(pos: number) {
  if (dragFrom !== null && dragFrom !== pos) moveColumn(dragFrom, pos);
  dragFrom = null;
}
</script>

<template>
  <section
    :aria-label="title || `Tabela de dados, variação ${variant}`"
    :class="`${S.root} ${className}`"
  >
    <div v-if="open" :class="S.overlay" @click="open = null" />
    <h3 v-if="title" :class="S.title">{{ title }}</h3>

    <div v-if="showToolbar" :class="S.toolbar">
      <div :class="S.search">
        <Icon name="search-01" :class-name="S.searchIcon" />
        <input
          v-model="query"
          type="search"
          :class="S.searchInput"
          :placeholder="searchPlaceholder"
          :aria-label="searchPlaceholder"
        />
      </div>

      <div :class="S.filterSelect">
        <select v-model="filterValue" :class="S.filterSelectInput" :aria-label="filterLabel">
          <option :value="ALL">{{ filterLabel }}</option>
          <option v-for="value in filterOptions" :key="value" :value="value">{{ value }}</option>
        </select>
        <Icon name="chevron-down" :class-name="S.filterSelectIcon" />
      </div>

      <div class="relative">
        <button
          type="button"
          :class="`${S.button} ${open === 'filter' ? S.buttonActive : ''}`"
          aria-label="Filtrar linhas"
          :aria-expanded="open === 'filter'"
          @click="open = open === 'filter' ? null : 'filter'"
        >
          <Icon name="filter" />
          Filter
        </button>
        <div v-if="open === 'filter'" :class="S.panelLeft">
          <label v-for="value in filterOptions" :key="value" :class="S.option">
            <input
              type="checkbox"
              :class="S.optionInput"
              :checked="filterValues.includes(value)"
              @change="toggleFilterValue(value)"
            />
            <span :class="S.optionLabel">{{ value }}</span>
          </label>
        </div>
      </div>

      <div class="relative">
        <button
          type="button"
          :class="`${S.button} ${open === 'columns' ? S.buttonActive : ''}`"
          aria-label="Gerir colunas"
          :aria-expanded="open === 'columns'"
          @click="open = open === 'columns' ? null : 'columns'"
        >
          <Icon name="layout-three-column" />
          Columns
        </button>
        <div v-if="open === 'columns'" :class="S.panelRight">
          <div
            v-for="(index, pos) in currentOrder"
            :key="index"
            :class="`${S.option} ${index === 0 ? S.optionDisabled : 'cursor-grab'}`"
            :draggable="index !== 0"
            @dragstart="dragFrom = pos"
            @dragover.prevent
            @drop="dropColumn(pos)"
          >
            <input
              type="checkbox"
              :class="S.optionInput"
              :checked="!hidden.includes(index)"
              :disabled="index === 0"
              :aria-label="`Mostrar ${cols[index]?.text ?? ''}`"
              @change="toggleColumn(index)"
            />
            <span :class="S.optionLabel">{{ cols[index]?.text }}</span>
            <span :class="S.dragHandle" aria-hidden="true"><Icon name="grip-vertical" /></span>
          </div>
        </div>
      </div>

      <div class="relative">
        <button
          type="button"
          :class="S.iconButton"
          aria-label="Mais acções"
          :aria-expanded="open === 'more'"
          @click="open = open === 'more' ? null : 'more'"
        >
          <Icon name="more-vertical" />
        </button>
        <div v-if="open === 'more'" :class="S.menu">
          <button v-for="action in moreActions" :key="action" type="button" :class="S.menuItem" @click="open = null">{{ action }}</button>
        </div>
      </div>
    </div>

    <div :class="S.viewport">
      <table :class="`${S.table} ${variant === '02' ? S.tableCompact : ''}`">
        <thead :class="S.head">
          <tr>
            <th v-if="variant !== '01'" :class="S.checkboxCell">
              <input type="checkbox" :class="S.checkbox" aria-label="Seleccionar todas as linhas" />
            </th>
            <th
              v-for="index in visibleCols" :key="index"
              :class="`${S.headCell} ${cols[index]?.className ?? ''}`"
            >
              <button
                type="button"
                :class="S.headCellButton"
                :aria-label="`Ordenar por ${cols[index]?.text}`"
                @click="sortAsc = !sortAsc"
              >
                <Text :text="cols[index]?.text ?? ''" />
                <span aria-hidden="true" :class="S.headCellSortIcon">↕</span>
              </button>
            </th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(row, ri) in chunkedRows" :key="ri" :class="S.row">
            <td v-if="variant !== '01'" :class="S.checkboxCell">
              <input
                type="checkbox"
                :class="S.checkbox"
                :checked="selected.includes(ri)"
                :aria-label="`Seleccionar ${row[0]?.text ?? ''}`"
                @change="toggle(ri)"
              />
            </td>
            <td
              v-for="index in visibleCols" :key="index"
              :class="`${S.cell} ${variant === '02' ? S.cellCompact : ''}`"
            >
              <a v-if="variant === '03' && index === 1" :href="`mailto:${row[index]?.text}`" class="text-slate-600 underline underline-offset-2">
                <Text v-bind="row[index] ?? { text: '' }" />
              </a>
              <span v-else-if="variant === '04' && index === 7" class="inline-flex gap-1">
                <button type="button" :class="S.actionButton">Invoice</button>
                <button type="button" :class="S.actionButton">Ledger</button>
              </span>
              <span v-else-if="variant === '01' && index === 6" class="relative inline-flex gap-1">
                <button type="button" :class="S.actionButton" aria-label="Ligar">📞</button>
                <button type="button" :class="S.actionButton" aria-label="Adicionar">📁</button>
                <button type="button" :class="S.actionButton" aria-label="Mais acções" @click="menuRow = menuRow === ri ? null : ri">⋯</button>
                <span v-if="menuRow === ri" :class="S.rowMenu">
                  <button v-for="a in ['Details', 'Offer a deal', 'Add to my list', 'Delete']" :key="a" type="button" :class="S.rowMenuItem">{{ a }}</button>
                </span>
              </span>
              <span v-else-if="variant === '03' && index === 6" class="inline-flex gap-1">
                <button type="button" :class="S.actionButton">Edit</button>
                <button type="button" :class="S.actionButton">Delete</button>
              </span>
              <span v-else :class="variant === '02' && index === 0 ? 'font-semibold' : ''">
                <Text v-bind="row[index] ?? { text: '' }" />
              </span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <footer v-if="variant !== '01' && variant !== '05'" :class="S.footer">
      <span>{{ variant === "02" ? "Page 1 of 13" : "Page 1 of 8" }}</span>
      <div :class="S.footerGroup">
        <label>Rows per page
          <select :class="S.footerSelect" aria-label="Linhas por página"><option>10</option><option>20</option></select>
        </label>
        <button type="button" :class="S.footerButton" aria-label="Página anterior">‹</button>
        <button type="button" :class="S.footerButton" aria-label="Página seguinte">›</button>
      </div>
    </footer>
  </section>
</template>
