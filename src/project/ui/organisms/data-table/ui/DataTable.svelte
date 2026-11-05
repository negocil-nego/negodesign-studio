<script lang="ts">
  import Icon from "../../../atoms/icon/ui/Icon.svelte";
  import { dataTableExampleByVariant } from "../data/examples";
  import type { DataTableProps, DataTableRow, DataTableVariant } from "../data/types";

  let {
    variant = "01",
    title,
    columns: suppliedColumns,
    rows: suppliedRows,
    className = "",
  }: DataTableProps = $props();

  let sortDirection = $state<"asc" | "desc">("asc");
  let selected = $state<string[]>([]);
  let menuRow = $state<string | null>("maya");

  const example = $derived(
    dataTableExampleByVariant[variant as DataTableVariant] ?? dataTableExampleByVariant["01"],
  );
  const columns = $derived(suppliedColumns ?? example.columns);
  const rows = $derived.by(() => {
    const data: DataTableRow[] = [...(suppliedRows ?? example.rows)];
    if (sortDirection === "desc") data.reverse();
    return data;
  });

  function toggleSelected(id: string) {
    selected = selected.includes(id)
      ? selected.filter((item) => item !== id)
      : [...selected, id];
  }
</script>

<section
  class={`min-w-0 overflow-hidden rounded-xl border border-zinc-200 bg-white text-zinc-800 shadow-sm ${className}`}
  aria-label={title ?? `Tabela de dados, variação ${variant}`}
>
  {#if title}<h3 class="px-4 pt-4 text-sm font-semibold">{title}</h3>{/if}
  <div class="overflow-x-auto">
    <table class={`w-full border-collapse text-left ${variant === "02" ? "text-[11px]" : "text-xs"}`}>
      <thead class="bg-zinc-50 text-[10px] uppercase tracking-wide text-zinc-500">
        <tr>
          {#if variant !== "01"}<th class="w-8 px-3 py-3"><input type="checkbox" aria-label="Seleccionar todas as linhas" /></th>{/if}
          {#each columns as column (column.key)}
            <th class="whitespace-nowrap border-b border-zinc-200 px-3 py-3 font-medium">
              <button
                class="inline-flex items-center gap-1"
                onclick={() => (sortDirection = sortDirection === "asc" ? "desc" : "asc")}
                aria-label={`Ordenar por ${column.label}`}
              >
                {column.label}<span aria-hidden="true" class="text-zinc-400">↕</span>
              </button>
            </th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each rows as row (row.id)}
          <tr class="border-b border-zinc-200 last:border-0 hover:bg-zinc-50">
            {#if variant !== "01"}<td class="px-3 py-2"><input type="checkbox" checked={selected.includes(row.id)} aria-label={`Seleccionar ${row.cells[0]}`} onchange={() => toggleSelected(row.id)} /></td>{/if}
            {#each columns as column, i (column.key)}
              {@const cell = row.cells[i] ?? ""}
              <td class={`whitespace-nowrap px-3 ${variant === "02" ? "py-2.5" : "py-2"}`}>
                {#if i === 0 && row.avatar}
                  <span class="flex items-center gap-2.5"><img class="h-8 w-8 rounded-full object-cover" src={row.avatar} alt="" /><span class="font-medium text-zinc-800">{cell}</span></span>
                {:else if variant === "03" && i === 1}
                  <a href={`mailto:${cell}`} class="text-slate-600 underline underline-offset-2">{cell}</a>
                {:else if (variant === "01" && i === 1) || (variant === "02" && i === 4)}
                  <span class={`rounded-full px-2 py-1 text-[10px] ${cell === "Urgent" ? "bg-red-50 text-red-600" : cell === "High" ? "bg-orange-50 text-orange-600" : "bg-emerald-50 text-emerald-700"}`}>{cell}</span>
                {:else if variant === "03" && i === 3}
                  <span class={`rounded-md border px-2 py-1 ${cell === "Active" ? "text-emerald-700" : "text-red-600"}`}>● {cell}</span>
                {:else if (variant === "03" && i === 5) || (variant === "04" && i === 6)}
                  <span class={`rounded px-2 py-1 ${cell === "Enabled" || cell === "Active" ? "bg-amber-100 text-amber-900" : "bg-rose-100 text-rose-700"}`}>{cell}</span>
                {:else if variant === "05" && i === 1 && cell !== "No Zones"}
                  <span class="rounded-full bg-emerald-100 px-2 py-1 text-emerald-700">{cell}</span>
                {:else if variant === "04" && i === 7}
                  <span class="inline-flex gap-1"><button class="rounded border px-2 py-1">Invoice</button><button class="rounded border px-2 py-1">Ledger</button></span>
                {:else if variant === "01" && i === 6}
                  <span class="relative inline-flex gap-1"><button class="rounded border px-2 py-1" aria-label="Ligar"><Icon name="Phone" /></button><button class="rounded border px-2 py-1" aria-label="Adicionar"><Icon name="FolderAdd" /></button><button class="rounded border px-2 py-1" aria-label="Mais acções" onclick={() => (menuRow = menuRow === row.id ? null : row.id)}><Icon name="MoreHorizontal" /></button>{#if menuRow === row.id}<span class="absolute right-0 top-full z-10 mt-1 w-36 rounded-lg border bg-white p-2 shadow-xl"><span class="block px-2 py-1 text-[9px] uppercase text-zinc-400">Menu</span>{#each ["Details", "Offer a deal", "Add to my list", "Delete"] as action}<button class="block w-full rounded px-2 py-1.5 text-left hover:bg-zinc-100">{action}</button>{/each}</span>{/if}</span>
                {:else if variant === "03" && i === 6}
                  <span class="inline-flex gap-1"><button class="rounded border px-2 py-1">Edit</button><button class="rounded border px-2 py-1">Delete</button></span>
                {:else}
                  <span class={variant === "02" && i === 0 ? "font-semibold" : ""}>{cell}</span>
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
      <div class="flex items-center gap-3"><label>Rows per page <select class="rounded border px-2 py-1" aria-label="Linhas por página"><option>10</option><option>20</option></select></label><button class="rounded border px-2 py-1" aria-label="Página anterior">‹</button><button class="rounded border px-2 py-1" aria-label="Página seguinte">›</button></div>
    </footer>
  {/if}
</section>
