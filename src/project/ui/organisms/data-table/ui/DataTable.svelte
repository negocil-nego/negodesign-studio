<script lang="ts">
  import Icon from "../../../atoms/icon/ui/Icon.svelte";
  import Text from "../../../atoms/text/ui/Text.svelte";
  import { dataTableExampleByVariant } from "../data/examples";
  import type { DataTableProps, DataTableVariant } from "../data/types";
  import type { TextProps } from "../../../atoms/text/data/types";

  let {
    variant = "01",
    title,
    columns: suppliedColumns,
    rows: suppliedRows,
    className = "",
  }: DataTableProps = $props();

  let sortAsc = $state(true);
  let selected = $state<number[]>([]);
  let menuRowIndex = $state<number | null>(null);

  const example = $derived(
    dataTableExampleByVariant[variant as DataTableVariant] ?? dataTableExampleByVariant["01"],
  );

  const columns = $derived<TextProps[]>(suppliedColumns ?? example.columns);

  /** Chunk flat rows array into 2-D by columns.length. */
  const rows = $derived.by(() => {
    const flat = suppliedRows ?? example.rows;
    const n = columns.length || 1;
    const result: TextProps[][] = [];
    for (let i = 0; i < flat.length; i += n) result.push(flat.slice(i, i + n));
    return sortAsc ? result : [...result].reverse();
  });

  function toggleSelected(i: number) {
    selected = selected.includes(i) ? selected.filter((x) => x !== i) : [...selected, i];
  }
</script>

<section
  class={`min-w-0 overflow-hidden rounded-xl border border-zinc-200 bg-white text-zinc-800 shadow-sm ${className}`}
  aria-label={title ?? `Tabela de dados, variação ${variant}`}
>
  {#if title}<h3 class="px-4 pt-4 text-sm font-semibold">{title}</h3>{/if}

  <div class="overflow-x-auto">
    <table class={`w-full border-collapse text-left ${variant === "02" ? "text-[11px]" : "text-xs"}`}>

      <!-- Head -->
      <thead class="bg-zinc-50 text-[10px] uppercase tracking-wide text-zinc-500">
        <tr>
          {#if variant !== "01"}
            <th class="w-8 px-3 py-3"><input type="checkbox" aria-label="Seleccionar todas as linhas" /></th>
          {/if}
          {#each columns as col, ci (ci)}
            <th class="whitespace-nowrap border-b border-zinc-200 px-3 py-3 font-medium {col.className ?? ''}">
              <button
                class="inline-flex items-center gap-1"
                onclick={() => (sortAsc = !sortAsc)}
                aria-label={`Ordenar por ${col.text}`}
              >
                <Text {...col} className="" />
                <span aria-hidden="true" class="text-zinc-400">↕</span>
              </button>
            </th>
          {/each}
        </tr>
      </thead>

      <!-- Body -->
      <tbody>
        {#each rows as row, ri (ri)}
          <tr class="border-b border-zinc-200 last:border-0 hover:bg-zinc-50">
            {#if variant !== "01"}
              <td class="px-3 py-2">
                <input
                  type="checkbox"
                  checked={selected.includes(ri)}
                  aria-label={`Seleccionar ${row[0]?.text ?? ""}`}
                  onchange={() => toggleSelected(ri)}
                />
              </td>
            {/if}

            {#each columns as _col, ci (ci)}
              {@const cell = row[ci] ?? { text: "" }}
              <td class={`whitespace-nowrap px-3 ${variant === "02" ? "py-2.5" : "py-2"}`}>

                {#if variant === "03" && ci === 1}
                  <!-- Email link -->
                  <a href={`mailto:${cell.text}`} class="text-slate-600 underline underline-offset-2">
                    <Text {...cell} />
                  </a>

                {:else if variant === "04" && ci === 7}
                  <!-- Invoice/Ledger actions -->
                  <span class="inline-flex gap-1">
                    <button class="rounded border px-2 py-1">Invoice</button>
                    <button class="rounded border px-2 py-1">Ledger</button>
                  </span>

                {:else if variant === "01" && ci === 6}
                  <!-- Row action menu -->
                  <span class="relative inline-flex gap-1">
                    <button class="rounded border px-2 py-1" aria-label="Ligar"><Icon name="Phone" /></button>
                    <button class="rounded border px-2 py-1" aria-label="Adicionar"><Icon name="FolderAdd" /></button>
                    <button
                      class="rounded border px-2 py-1"
                      aria-label="Mais acções"
                      onclick={() => (menuRowIndex = menuRowIndex === ri ? null : ri)}
                    ><Icon name="MoreHorizontal" /></button>
                    {#if menuRowIndex === ri}
                      <span class="absolute right-0 top-full z-10 mt-1 w-36 rounded-lg border bg-white p-2 shadow-xl">
                        <span class="block px-2 py-1 text-[9px] uppercase text-zinc-400">Menu</span>
                        {#each ["Details", "Offer a deal", "Add to my list", "Delete"] as action}
                          <button class="block w-full rounded px-2 py-1.5 text-left hover:bg-zinc-100">{action}</button>
                        {/each}
                      </span>
                    {/if}
                  </span>

                {:else if variant === "03" && ci === 6}
                  <!-- Edit/Delete actions -->
                  <span class="inline-flex gap-1">
                    <button class="rounded border px-2 py-1">Edit</button>
                    <button class="rounded border px-2 py-1">Delete</button>
                  </span>

                {:else}
                  <!-- Generic: Text atom handles className (badges etc.) -->
                  <span class={variant === "02" && ci === 0 ? "font-semibold" : ""}>
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
    <footer class="flex items-center justify-between border-t border-zinc-200 px-4 py-3 text-[11px] text-zinc-500">
      <span>{variant === "02" ? "Page 1 of 13" : "Page 1 of 8"}</span>
      <div class="flex items-center gap-3">
        <label>Rows per page
          <select class="rounded border px-2 py-1" aria-label="Linhas por página">
            <option>10</option><option>20</option>
          </select>
        </label>
        <button class="rounded border px-2 py-1" aria-label="Página anterior">‹</button>
        <button class="rounded border px-2 py-1" aria-label="Página seguinte">›</button>
      </div>
    </footer>
  {/if}
</section>
