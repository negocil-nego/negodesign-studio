import { useMemo, useState } from "react";
import Text from "../../../atoms/text/ui/Text";
import { dataTableExampleByVariant } from "../data/examples";
import type { DataTableProps, DataTableVariant } from "../data/types";
import type { TextProps } from "../../../atoms/text/data/types";

function flatText(cell: TextProps): string { return cell.text; }

export default function DataTable({ variant = "01", title, columns, rows, className = "" }: DataTableProps) {
  const [sortAsc, setSortAsc] = useState(true);
  const [selected, setSelected] = useState<number[]>([]);
  const [menuRow, setMenuRow] = useState<number | null>(null);

  const example = dataTableExampleByVariant[variant as DataTableVariant] ?? dataTableExampleByVariant["01"];
  const cols: TextProps[] = columns ?? example.columns;

  const chunkedRows = useMemo(() => {
    const flat = rows ?? example.rows;
    const n = cols.length || 1;
    const result: TextProps[][] = [];
    for (let i = 0; i < flat.length; i += n) result.push(flat.slice(i, i + n));
    return sortAsc ? result : [...result].reverse();
  }, [rows, example.rows, cols.length, sortAsc]);

  function toggle(i: number) {
    setSelected((prev) => prev.includes(i) ? prev.filter((x) => x !== i) : [...prev, i]);
  }

  return (
    <section
      aria-label={title ?? `Tabela de dados, variação ${variant}`}
      className={`min-w-0 overflow-hidden rounded-xl border border-zinc-200 bg-white text-zinc-800 shadow-sm ${className}`}
    >
      {title && <h3 className="px-4 pt-4 text-sm font-semibold">{title}</h3>}
      <div className="overflow-x-auto">
        <table className={`w-full border-collapse text-left ${variant === "02" ? "text-[11px]" : "text-xs"}`}>
          <thead className="bg-zinc-50 text-[10px] uppercase tracking-wide text-zinc-500">
            <tr>
              {variant !== "01" && <th className="w-8 px-3 py-3"><input type="checkbox" aria-label="Seleccionar todas as linhas" /></th>}
              {cols.map((col, ci) => (
                <th key={ci} className={`whitespace-nowrap border-b border-zinc-200 px-3 py-3 font-medium ${col.className ?? ""}`}>
                  <button className="inline-flex items-center gap-1" onClick={() => setSortAsc(!sortAsc)} aria-label={`Ordenar por ${col.text}`}>
                    <Text text={col.text} />
                    <span aria-hidden className="text-zinc-400">↕</span>
                  </button>
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {chunkedRows.map((row, ri) => (
              <tr key={ri} className="border-b border-zinc-200 last:border-0 hover:bg-zinc-50">
                {variant !== "01" && (
                  <td className="px-3 py-2">
                    <input type="checkbox" checked={selected.includes(ri)} aria-label={`Seleccionar ${row[0]?.text ?? ""}`} onChange={() => toggle(ri)} />
                  </td>
                )}
                {cols.map((_col, ci) => {
                  const cell = row[ci] ?? { text: "" };
                  return (
                    <td key={ci} className={`whitespace-nowrap px-3 ${variant === "02" ? "py-2.5" : "py-2"}`}>
                      {variant === "03" && ci === 1 ? (
                        <a href={`mailto:${flatText(cell)}`} className="text-slate-600 underline underline-offset-2"><Text {...cell} /></a>
                      ) : variant === "04" && ci === 7 ? (
                        <span className="inline-flex gap-1"><button className="rounded border px-2 py-1">Invoice</button><button className="rounded border px-2 py-1">Ledger</button></span>
                      ) : variant === "01" && ci === 6 ? (
                        <span className="relative inline-flex gap-1">
                          <button className="rounded border px-2 py-1" aria-label="Ligar">📞</button>
                          <button className="rounded border px-2 py-1" aria-label="Adicionar">📁</button>
                          <button className="rounded border px-2 py-1" onClick={() => setMenuRow(menuRow === ri ? null : ri)}>⋯</button>
                          {menuRow === ri && (
                            <span className="absolute right-0 top-full z-10 mt-1 w-36 rounded-lg border bg-white p-2 shadow-xl">
                              {["Details", "Offer a deal", "Add to my list", "Delete"].map((a) => (
                                <button key={a} className="block w-full rounded px-2 py-1.5 text-left hover:bg-zinc-100">{a}</button>
                              ))}
                            </span>
                          )}
                        </span>
                      ) : variant === "03" && ci === 6 ? (
                        <span className="inline-flex gap-1"><button className="rounded border px-2 py-1">Edit</button><button className="rounded border px-2 py-1">Delete</button></span>
                      ) : (
                        <span className={variant === "02" && ci === 0 ? "font-semibold" : ""}><Text {...cell} /></span>
                      )}
                    </td>
                  );
                })}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      {variant !== "01" && variant !== "05" && (
        <footer className="flex items-center justify-between border-t border-zinc-200 px-4 py-3 text-[11px] text-zinc-500">
          <span>{variant === "02" ? "Page 1 of 13" : "Page 1 of 8"}</span>
          <div className="flex items-center gap-3">
            <label>Rows per page <select className="rounded border px-2 py-1"><option>10</option><option>20</option></select></label>
            <button className="rounded border px-2 py-1">‹</button>
            <button className="rounded border px-2 py-1">›</button>
          </div>
        </footer>
      )}
    </section>
  );
}
