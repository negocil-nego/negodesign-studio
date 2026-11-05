import { useMemo, useState } from "react";
import { dataTableExampleByVariant } from "../data/examples";
import type { DataTableProps, DataTableVariant } from "../data/types";

export default function DataTable({ variant = "01", title, columns, rows, className = "" }: DataTableProps) {
  const [reverse, setReverse] = useState(false);
  const [selected, setSelected] = useState<string[]>([]);
  const example = dataTableExampleByVariant[variant as DataTableVariant] ?? dataTableExampleByVariant["01"];
  const visibleColumns = columns ?? example.columns;
  const visibleRows = useMemo(() => {
    const data = [...(rows ?? example.rows)];
    return reverse ? data.reverse() : data;
  }, [rows, example.rows, reverse]);

  return (
    <section aria-label={title ?? `Data table variation ${variant}`} className={`min-w-0 overflow-hidden rounded-xl border border-zinc-200 bg-white text-zinc-800 shadow-sm ${className}`}>
      {title && <h3 className="px-4 pt-4 text-sm font-semibold">{title}</h3>}
      <div className="overflow-x-auto"><table className="w-full border-collapse text-left text-xs">
        <thead className="bg-zinc-50 text-[10px] uppercase tracking-wide text-zinc-500"><tr>
          {variant !== "01" && <th className="px-3 py-3"><input type="checkbox" aria-label="Select all rows" /></th>}
          {visibleColumns.map((column) => <th key={column.key} className="whitespace-nowrap border-b px-3 py-3 font-medium"><button onClick={() => setReverse(!reverse)}>{column.label} ↕</button></th>)}
        </tr></thead>
        <tbody>{visibleRows.map((row) => <tr key={row.id} className="border-b last:border-0 hover:bg-zinc-50">
          {variant !== "01" && <td className="px-3 py-2"><input type="checkbox" checked={selected.includes(row.id)} onChange={() => setSelected(selected.includes(row.id) ? selected.filter((id) => id !== row.id) : [...selected, row.id])} aria-label={`Select ${row.cells[0]}`} /></td>}
          {visibleColumns.map((column, index) => <td key={`${row.id}-${column.key}`} className="whitespace-nowrap px-3 py-2">{index === 0 && row.avatar ? <span className="flex items-center gap-2"><img className="h-8 w-8 rounded-full object-cover" src={row.avatar} alt="" />{row.cells[index]}</span> : row.cells[index]}</td>)}
        </tr>)}</tbody>
      </table></div>
      {variant !== "01" && variant !== "05" && <footer className="flex justify-between border-t px-4 py-3 text-[11px] text-zinc-500"><span>Page 1 of 13</span><div className="flex gap-2"><button className="rounded border px-2">‹</button><button className="rounded border px-2">›</button></div></footer>}
    </section>
  );
}
