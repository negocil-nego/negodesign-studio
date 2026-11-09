import { useMemo, useState } from "react";
import Icon from "../../../atoms/icon/ui/Icon";
import Text from "../../../atoms/text/ui/Text";
import { StyleComponent } from "../../../../data/style.component";
import { dataTableExampleByVariant } from "../data/examples";
import DataTableFilter from "../shared/data-table-filter/ui/DataTableFilter";
import type { DataTableProps, DataTableVariant } from "../data/types";
import type { TextProps } from "../../../atoms/text/data/types";

export default function DataTable({ variant = "01", title, columns, rows, className = "" }: DataTableProps) {
  const S = StyleComponent.dataTable;

  const [searchQuery, setSearchQuery] = useState("");
  const [categoryFilter, setCategoryFilter] = useState("all");
  const [sortAsc, setSortAsc] = useState(true);
  const [selected, setSelected] = useState<number[]>([]);
  const [menuRow, setMenuRow] = useState<number | null>(null);
  const [hiddenColumns, setHiddenColumns] = useState<number[]>([]);

  const example = dataTableExampleByVariant[variant as DataTableVariant] ?? dataTableExampleByVariant["01"];
  const cols: TextProps[] = columns ?? example.columns;

  const allChunkedRows = useMemo(() => {
    const flat = rows ?? example.rows;
    const n = cols.length || 1;
    const result: TextProps[][] = [];
    for (let i = 0; i < flat.length; i += n) result.push(flat.slice(i, i + n));
    return result;
  }, [rows, example.rows, cols.length]);

  const filterOptions = useMemo(() => {
    const colIdx = cols.length > 1 ? 1 : 0;
    const values = new Set<string>();
    for (const row of allChunkedRows) {
      const val = row[colIdx]?.text?.trim();
      if (val) values.add(val);
    }
    return [...values];
  }, [allChunkedRows, cols.length]);

  const visibleRows = useMemo(() => {
    let list = sortAsc ? [...allChunkedRows] : [...allChunkedRows].reverse();
    if (categoryFilter !== "all") {
      const colIdx = cols.length > 1 ? 1 : 0;
      list = list.filter((row) => row[colIdx]?.text === categoryFilter);
    }
    if (searchQuery.trim()) {
      const q = searchQuery.toLowerCase();
      list = list.filter((row) => row.some((c) => c.text.toLowerCase().includes(q)));
    }
    return list;
  }, [allChunkedRows, sortAsc, categoryFilter, searchQuery, cols.length]);

  function toggleCol(i: number) {
    if (i === 0) return;
    setHiddenColumns((prev) =>
      prev.includes(i) ? prev.filter((x) => x !== i) : [...prev, i],
    );
  }

  function toggleSelected(i: number) {
    setSelected((prev) => (prev.includes(i) ? prev.filter((x) => x !== i) : [...prev, i]));
  }

  function resetFilters() {
    setSearchQuery("");
    setCategoryFilter("all");
    setHiddenColumns([]);
  }

  return (
    <section aria-label={title ?? `Tabela de dados, variação ${variant}`} className={`${S.root} ${className}`}>
      {title && <h3 className={S.title}>{title}</h3>}

      <DataTableFilter
        columns={cols}
        filterOptions={filterOptions}
        searchQuery={searchQuery}
        categoryFilter={categoryFilter}
        hiddenColumns={hiddenColumns}
        onSearchChange={setSearchQuery}
        onCategoryChange={setCategoryFilter}
        onToggleColumn={toggleCol}
        onReset={resetFilters}
      />

      <div className={S.viewport}>
        <table className={`${S.table} ${variant === "02" ? S.tableCompact : ""}`}>
          <thead className={S.head}>
            <tr>
              {variant !== "01" && (
                <th className={S.checkboxCell}>
                  <input type="checkbox" className={S.checkbox} aria-label="Seleccionar todas" />
                </th>
              )}
              {cols.map((col, ci) => !hiddenColumns.includes(ci) && (
                <th key={ci} className={`${S.headCell} ${col.className ?? ""}`}>
                  <button
                    className={S.headCellButton}
                    onClick={() => setSortAsc(!sortAsc)}
                    aria-label={`Ordenar por ${col.text}`}
                  >
                    <Text text={col.text} />
                    <span aria-hidden className={S.headCellSortIcon}>↕</span>
                  </button>
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {visibleRows.map((row, ri) => (
              <tr key={ri} className={S.row}>
                {variant !== "01" && (
                  <td className="px-3 py-2">
                    <input
                      type="checkbox"
                      className={S.checkbox}
                      checked={selected.includes(ri)}
                      onChange={() => toggleSelected(ri)}
                    />
                  </td>
                )}
                {cols.map((_, ci) => !hiddenColumns.includes(ci) && (() => {
                  const cell = row[ci] ?? { text: "" };
                  return (
                    <td key={ci} className={`${S.cell} ${variant === "02" ? S.cellCompact : ""}`}>
                      {variant === "03" && ci === 1 ? (
                        <a href={`mailto:${cell.text}`} className="text-blue-400 underline underline-offset-2"><Text {...cell} /></a>
                      ) : variant === "04" && ci === 7 ? (
                        <span className="inline-flex gap-1"><button className={S.actionButton}>Invoice</button><button className={S.actionButton}>Ledger</button></span>
                      ) : variant === "01" && ci === 6 ? (
                        <span className="relative inline-flex gap-1">
                          <button className={S.actionButton} aria-label="Ligar"><Icon name="Phone" /></button>
                          <button className={S.actionButton} aria-label="Adicionar"><Icon name="FolderAdd" /></button>
                          <button className={S.actionButton} onClick={() => setMenuRow(menuRow === ri ? null : ri)}><Icon name="MoreHorizontal" /></button>
                          {menuRow === ri && (
                            <span className={S.rowMenu}>
                              {["Details", "Offer a deal", "Add to my list", "Delete"].map((a) => (
                                <button key={a} className={S.rowMenuItem}>{a}</button>
                              ))}
                            </span>
                          )}
                        </span>
                      ) : variant === "03" && ci === 6 ? (
                        <span className="inline-flex gap-1"><button className={S.actionButton}>Edit</button><button className={S.actionButton}>Delete</button></span>
                      ) : (
                        <span className={variant === "02" && ci === 0 ? "font-semibold text-zinc-100" : "text-zinc-200"}><Text {...cell} /></span>
                      )}
                    </td>
                  );
                })())}
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {variant !== "01" && variant !== "05" && (
        <footer className={S.footer}>
          <span>{variant === "02" ? "Page 1 of 13" : "Page 1 of 8"}</span>
          <div className={S.footerGroup}>
            <label className="flex items-center gap-2">
              Rows per page
              <select className={S.footerSelect} aria-label="Linhas por página">
                <option>10</option>
                <option>20</option>
              </select>
            </label>
            <button className={S.footerButton}>‹</button>
            <button className={S.footerButton}>›</button>
          </div>
        </footer>
      )}
    </section>
  );
}
