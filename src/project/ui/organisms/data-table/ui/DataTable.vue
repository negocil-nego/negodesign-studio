<script setup lang="ts">
import { computed, ref } from "vue";
import { dataTableExampleByVariant } from "../data/examples";
import type { DataTableProps, DataTableVariant } from "../data/types";

const props = withDefaults(defineProps<DataTableProps>(), { variant: "01", title: "", className: "" });
const reverse = ref(false);
const selected = ref<string[]>([]);
const example = computed(() => dataTableExampleByVariant[props.variant as DataTableVariant] ?? dataTableExampleByVariant["01"]);
const visibleColumns = computed(() => props.columns ?? example.value.columns);
const visibleRows = computed(() => {
  const data = [...(props.rows ?? example.value.rows)];
  return reverse.value ? data.reverse() : data;
});
function toggleRow(id: string) {
  selected.value = selected.value.includes(id) ? selected.value.filter((item) => item !== id) : [...selected.value, id];
}
</script>
<template>
  <section :aria-label="title || `Data table variation ${variant}`" :class="`min-w-0 overflow-hidden rounded-xl border border-zinc-200 bg-white text-zinc-800 shadow-sm ${className}`">
    <h3 v-if="title" class="px-4 pt-4 text-sm font-semibold">{{ title }}</h3>
    <div class="overflow-x-auto"><table class="w-full border-collapse text-left text-xs">
      <thead class="bg-zinc-50 text-[10px] uppercase tracking-wide text-zinc-500"><tr>
        <th v-if="variant !== '01'" class="px-3 py-3"><input type="checkbox" aria-label="Select all rows" /></th>
        <th v-for="column in visibleColumns" :key="column.key" class="whitespace-nowrap border-b px-3 py-3 font-medium"><button @click="reverse = !reverse">{{ column.label }} ↕</button></th>
      </tr></thead>
      <tbody><tr v-for="row in visibleRows" :key="row.id" class="border-b last:border-0 hover:bg-zinc-50">
        <td v-if="variant !== '01'" class="px-3 py-2"><input type="checkbox" :checked="selected.includes(row.id)" :aria-label="`Select ${row.cells[0]}`" @change="toggleRow(row.id)" /></td>
        <td v-for="(column, index) in visibleColumns" :key="`${row.id}-${column.key}`" class="whitespace-nowrap px-3 py-2"><span v-if="index === 0 && row.avatar" class="flex items-center gap-2"><img class="h-8 w-8 rounded-full object-cover" :src="row.avatar" alt="" />{{ row.cells[index] }}</span><template v-else>{{ row.cells[index] }}</template></td>
      </tr></tbody>
    </table></div>
    <footer v-if="variant !== '01' && variant !== '05'" class="flex justify-between border-t px-4 py-3 text-[11px] text-zinc-500"><span>Page 1 of 13</span><div class="flex gap-2"><button class="rounded border px-2">‹</button><button class="rounded border px-2">›</button></div></footer>
  </section>
</template>
