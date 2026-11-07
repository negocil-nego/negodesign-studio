<script setup lang="ts">
import { computed, ref } from "vue";
import Text from "../../../atoms/text/ui/Text.vue";
import { dataTableExampleByVariant } from "../data/examples";
import type { DataTableProps, DataTableVariant } from "../data/types";
import type { TextProps } from "../../../atoms/text/data/types";

const props = withDefaults(defineProps<DataTableProps>(), { variant: "01", className: "" });
const sortAsc = ref(true);
const selected = ref<number[]>([]);
const menuRow = ref<number | null>(null);

const example = computed(() => dataTableExampleByVariant[props.variant as DataTableVariant] ?? dataTableExampleByVariant["01"]);
const cols = computed<TextProps[]>(() => props.columns ?? example.value.columns);

const chunkedRows = computed<TextProps[][]>(() => {
  const flat = props.rows ?? example.value.rows;
  const n = cols.value.length || 1;
  const result: TextProps[][] = [];
  for (let i = 0; i < flat.length; i += n) result.push(flat.slice(i, i + n));
  return sortAsc.value ? result : [...result].reverse();
});

function toggle(i: number) {
  selected.value = selected.value.includes(i)
    ? selected.value.filter((x) => x !== i)
    : [...selected.value, i];
}
</script>

<template>
  <section
    :aria-label="title || `Tabela de dados, variação ${variant}`"
    :class="`min-w-0 overflow-hidden rounded-xl border border-zinc-200 bg-white text-zinc-800 shadow-sm ${className}`"
  >
    <h3 v-if="title" class="px-4 pt-4 text-sm font-semibold">{{ title }}</h3>
    <div class="overflow-x-auto">
      <table :class="`w-full border-collapse text-left ${variant === '02' ? 'text-[11px]' : 'text-xs'}`">
        <thead class="bg-zinc-50 text-[10px] uppercase tracking-wide text-zinc-500">
          <tr>
            <th v-if="variant !== '01'" class="w-8 px-3 py-3">
              <input type="checkbox" aria-label="Seleccionar todas as linhas" />
            </th>
            <th
              v-for="(col, ci) in cols" :key="ci"
              :class="`whitespace-nowrap border-b border-zinc-200 px-3 py-3 font-medium ${col.className ?? ''}`"
            >
              <button
                class="inline-flex items-center gap-1"
                :aria-label="`Ordenar por ${col.text}`"
                @click="sortAsc = !sortAsc"
              >
                <Text :text="col.text" />
                <span aria-hidden="true" class="text-zinc-400">↕</span>
              </button>
            </th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(row, ri) in chunkedRows" :key="ri" class="border-b border-zinc-200 last:border-0 hover:bg-zinc-50">
            <td v-if="variant !== '01'" class="px-3 py-2">
              <input type="checkbox" :checked="selected.includes(ri)" :aria-label="`Seleccionar ${row[0]?.text ?? ''}`" @change="toggle(ri)" />
            </td>
            <td
              v-for="(col, ci) in cols" :key="ci"
              :class="`whitespace-nowrap px-3 ${variant === '02' ? 'py-2.5' : 'py-2'}`"
            >
              <a v-if="variant === '03' && ci === 1" :href="`mailto:${row[ci]?.text}`" class="text-slate-600 underline underline-offset-2">
                <Text v-bind="row[ci] ?? { text: '' }" />
              </a>
              <span v-else-if="variant === '04' && ci === 7" class="inline-flex gap-1">
                <button class="rounded border px-2 py-1">Invoice</button>
                <button class="rounded border px-2 py-1">Ledger</button>
              </span>
              <span v-else-if="variant === '01' && ci === 6" class="relative inline-flex gap-1">
                <button class="rounded border px-2 py-1" aria-label="Ligar">📞</button>
                <button class="rounded border px-2 py-1" aria-label="Adicionar">📁</button>
                <button class="rounded border px-2 py-1" @click="menuRow = menuRow === ri ? null : ri">⋯</button>
                <span v-if="menuRow === ri" class="absolute right-0 top-full z-10 mt-1 w-36 rounded-lg border bg-white p-2 shadow-xl">
                  <span class="block px-2 py-1 text-[9px] uppercase text-zinc-400">Menu</span>
                  <button v-for="a in ['Details', 'Offer a deal', 'Add to my list', 'Delete']" :key="a" class="block w-full rounded px-2 py-1.5 text-left hover:bg-zinc-100">{{ a }}</button>
                </span>
              </span>
              <span v-else-if="variant === '03' && ci === 6" class="inline-flex gap-1">
                <button class="rounded border px-2 py-1">Edit</button>
                <button class="rounded border px-2 py-1">Delete</button>
              </span>
              <span v-else :class="variant === '02' && ci === 0 ? 'font-semibold' : ''">
                <Text v-bind="row[ci] ?? { text: '' }" />
              </span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <footer v-if="variant !== '01' && variant !== '05'" class="flex items-center justify-between border-t border-zinc-200 px-4 py-3 text-[11px] text-zinc-500">
      <span>{{ variant === "02" ? "Page 1 of 13" : "Page 1 of 8" }}</span>
      <div class="flex items-center gap-3">
        <label>Rows per page <select class="rounded border px-2 py-1"><option>10</option><option>20</option></select></label>
        <button class="rounded border px-2 py-1">‹</button>
        <button class="rounded border px-2 py-1">›</button>
      </div>
    </footer>
  </section>
</template>
