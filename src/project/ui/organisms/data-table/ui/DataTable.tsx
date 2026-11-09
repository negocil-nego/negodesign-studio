import { useMemo, useRef, useState } from "react";
import Icon from "../../../atoms/icon/ui/Icon";
import Text from "../../../atoms/text/ui/Text";
import { StyleComponent } from "../../../data/style.component";
import { dataTableExampleByVariant } from "../data/examples";
import type { DataTableProps, DataTableVariant } from "../data/types";
import type { TextProps } from "../../../atoms/text/data/types";

const S = StyleComponent.dataTable;
const ALL = "__all__";

export default function DataTable({
  variant = "01",
  title,
  columns,
  rows,
  className = "",
  searchPlaceholder = "Search",
  filterColumn = 0,
  filterAllLabel,
  showToolbar = true,
  moreActions = ["Export CSV", "Print"],
}: DataTableProps) {
  const [sortAsc, setSortAsc] = useState(true);
  const [selected, setSelected] = useState<number[]>([]);
  const [menuRow, setMenuRow] = useState<number | null>(null);
  const [query, setQuery] = useState("");
  const [filterValue, setFilterValue] = useState(ALL);
  const [filterValues, setFilterValues] = useState<string[]>([]);
  const [hidden, setHidden] = useState<number[]>([]);
  const [order, setOrder] = useState<number[] | null>(null);
  const [open, setOpen] = useState<"filter" | "columns" | "more" | null>(null);
  const dragFrom = useRef<number | null>(null);

  const example = dataTableExampleByVariant[variant as DataTableVariant] ?? dataTableExampleByVariant["01"];
  const cols: TextProps[] = columns ?? example.columns;

  const allRows = useMemo(() => {
    const flat = rows ?? example.rows;
    const n = cols.length || 1;
    const result: TextProps[][] = [];
    for (let i = 0; i < flat.length; i += n) result.push(flat.slice(i, i + n));
    return result;
  }, [rows, example.rows, cols.length]);

  const filterOptions = useMemo(() => {
    const seen: string[] = [];
    for (const row of allRows) {
      const value = row[filterColumn]?.text ?? "";
      if (value && !seen.includes(value)) seen.push(value);
    }
    return seen;
  }, [allRows, filterColumn]);

  const chunkedRows = useMemo(() => {
    const q = query.trim().toLowerCase();
    const filtered = allRows.filter((row) => {
      const value = row[filterColumn]?.text ?? "";
      if (filterValue !== ALL && value !== filterValue) return false;
      if (filterValues.length > 0 && !filterValues.includes(value)) return false;
      if (q && !row.some((cell) => cell.text.toLowerCase().includes(q))) return false;
      return true;
    });
    return sortAsc ? filtered : [...filtered].reverse();
  }, [allRows, sortAsc, query, filterValue, filterValues, filterColumn]);

  const currentOrder = order && order.length === cols.length ? order : cols.map((_, i) => i);
  const visibleCols = currentOrder.filter((i) => !hidden.includes(i));
  const filterLabel = filterAllLabel ?? `All ${cols[filterColumn]?.text ?? ""}`;

  function toggle(i: number) {
    setSelected((prev) => prev.includes(i) ? prev.filter((x) => x !== i) : [...prev, i]);
  }

  function toggleColumn(index: number) {
    if (index === 0) return;
    setHidden((prev) => prev.includes(index) ? prev.filter((x) => x !== index) : [...prev, index]);
  }

  function toggleFilterValue(value: string) {
    setFilterValues((prev) => prev.includes(value) ? prev.filter((x) => x !== value) : [...prev, value]);
  }

  function moveColumn(from: number, to: number) {
    const next = [...currentOrder];
    const [moved] = next.splice(from, 1);
    next.splice(to, 0, moved);
    setOrder(next);
  }

  return (
    <section
      aria-label={title ?? `Tabela de dados, variação ${variant}`}
      className={`${S.root} ${className}`}
    >
      {open && <div className={S.overlay} onClick={() => setOpen(null)} />}
      {title && <h3 className={S.title}>{title}</h3>}

      {showToolbar && (
        <div className={S.toolbar}>
          <div className={S.search}>
            <Icon name="search-01" className={S.searchIcon} />
            <input
              type="search"
              className={S.searchInput}
              placeholder={searchPlaceholder}
              aria-label={searchPlaceholder}
              value={query}
              onChange={(e) => setQuery(e.target.value)}
            />
          </div>

          <div className={S.filterSelect}>
            <select
              className={S.filterSelectInput}
              aria-label={filterLabel}
              value={filterValue}
              onChange={(e) => setFilterValue(e.target.value)}
            >
              <option value={ALL}>{filterLabel}</option>
              {filterOptions.map((value) => (
                <option key={value} value={value}>{value}</option>
              ))}
            </select>
            <Icon name="chevron-down" className={S.filterSelectIcon} />
          </div>

          <div className="relative">
            <button
              type="button"
              className={`${S.button} ${open === "filter" ? S.buttonActive : ""}`}
              aria-label="Filtrar linhas"
              aria-expanded={open === "filter"}
              onClick={() => setOpen(open === "filter" ? null : "filter")}
            >
              <Icon name="filter" />
              Filter
            </button>
            {open === "filter" && (
              <div className={S.panelLeft}>
                {filterOptions.map((value) => (
                  <label key={value} className={S.option}>
                    <input
                      type="checkbox"
                      className={S.optionInput}
                      checked={filterValues.includes(value)}
                      onChange={() => toggleFilterValue(value)}
                    />
                    <span className={S.optionLabel}>{value}</span>
                  </label>
                ))}
              </div>
            )}
          </div>

          <div className="relative">
            <button
              type="button"
              className={`${S.button} ${open === "columns" ? S.buttonActive : ""}`}
              aria-label="Gerir colunas"
              aria-expanded={open === "columns"}
              onClick={() => setOpen(open === "columns" ? null : "columns")}
            >
              <Icon name="layout-three-column" />
              Columns
            </button>
            {open === "columns" && (
              <div className={S.panelRight}>
                {currentOrder.map((index, pos) => {
                  const locked = index === 0;
                  return (
                    <div
                      key={index}
                      className={`${S.option} ${locked ? S.optionDisabled : "cursor-grab"}`}
                      draggable={!locked}
                      onDragStart={() => (dragFrom.current = pos)}
                      onDragOver={(e) => e.preventDefault()}
                      onDrop={() => {
                        if (dragFrom.current !== null && dragFrom.current !== pos) moveColumn(dragFrom.current, pos);
                        dragFrom.current = null;
                      }}
                    >
                      <input
                        type="checkbox"
                        className={S.optionInput}
                        checked={!hidden.includes(index)}
                        disabled={locked}
                        aria-label={`Mostrar ${cols[index]?.text ?? ""}`}
                        onChange={() => toggleColumn(index)}
                      />
                      <span className={S.optionLabel}>{cols[index]?.text}</span>
                      <span className={S.dragHandle} aria-hidden="true"><Icon name="grip-vertical" /></span>
                    </div>
                  );
                })}
              </div>
            )}
          </div>

          <div className="relative">
            <button
              type="button"
              className={S.iconButton}
              aria-label="Mais acções"
              aria-expanded={open === "more"}
              onClick={() => setOpen(open === "more" ? null : "more")}
            >
              <Icon name="more-vertical" />
            </button>
            {open === "more" && (
              <div className={S.menu}>
                {moreActions.map((action) => (
                  <button key={action} type="button" className={S.menuItem} onClick={() => setOpen(null)}>{action}</button>
                ))}
              </div>
            )}
          </div>
        </div>
      )}

      <div className={S.viewport}>
        <table className={`${S.table} ${variant === "02" ? S.tableCompact : ""}`}>
          <thead className={S.head}>
            <tr>
              {variant !== "01" && (
                <th className={S.checkboxCell}>
                  <input type="checkbox" className={S.checkbox} aria-label="Seleccionar todas as linhas" />
                </th>
              )}
              {visibleCols.map((index) => (
                <th key={index} className={`${S.headCell} ${cols[index]?.className ?? ""}`}>
                  <button
                    type="button"
                    className={S.headCellButton}
                    onClick={() => setSortAsc(!sortAsc)}
                    aria-label={`Ordenar por ${cols[index]?.text}`}
                  >
                    <Text text={cols[index]?.text ?? ""} />
                    <span aria-hidden className={S.headCellSortIcon}>↕</span>
                  </button>
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {chunkedRows.map((row, ri) => (
              <tr key={ri} className={S.row}>
                {variant !== "01" && (
                  <td className={S.checkboxCell}>
                    <input
                      type="checkbox"
                      className={S.checkbox}
                      checked={selected.includes(ri)}
                      aria-label={`Seleccionar ${row[0]?.text ?? ""}`}
                      onChange={() => toggle(ri)}
                    />
                  </td>
                )}
                {visibleCols.map((index) => {
                  const cell = row[index] ?? { text: "" };
                  return (
                    <td key={index} className={`${S.cell} ${variant === "02" ? S.cellCompact : ""}`}>
                      {variant === "03" && index === 1 ? (
                        <a href={`mailto:${cell.text}`} className="text-slate-600 underline underline-offset-2"><Text {...cell} /></a>
                      ) : variant === "04" && index === 7 ? (
                        <span className="inline-flex gap-1">
                          <button type="button" className={S.actionButton}>Invoice</button>
                          <button type="button" className={S.actionButton}>Ledger</button>
                        </span>
                      ) : variant === "01" && index === 6 ? (
                        <span className="relative inline-flex gap-1">
                          <button type="button" className={S.actionButton} aria-label="Ligar">📞</button>
                          <button type="button" className={S.actionButton} aria-label="Adicionar">📁</button>
                          <button type="button" className={S.actionButton} aria-label="Mais acções" onClick={() => setMenuRow(menuRow === ri ? null : ri)}>⋯</button>
                          {menuRow === ri && (
                            <span className={S.rowMenu}>
                              {["Details", "Offer a deal", "Add to my list", "Delete"].map((a) => (
                                <button key={a} type="button" className={S.rowMenuItem}>{a}</button>
                              ))}
                            </span>
                          )}
                        </span>
                      ) : variant === "03" && index === 6 ? (
                        <span className="inline-flex gap-1">
                          <button type="button" className={S.actionButton}>Edit</button>
                          <button type="button" className={S.actionButton}>Delete</button>
                        </span>
                      ) : (
                        <span className={variant === "02" && index === 0 ? "font-semibold" : ""}><Text {...cell} /></span>
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
        <footer className={S.footer}>
          <span>{variant === "02" ? "Page 1 of 13" : "Page 1 of 8"}</span>
          <div className={S.footerGroup}>
            <label>Rows per page
              <select className={S.footerSelect} aria-label="Linhas por página"><option>10</option><option>20</option></select>
            </label>
            <button type="button" className={S.footerButton} aria-label="Página anterior">‹</button>
            <button type="button" className={S.footerButton} aria-label="Página seguinte">›</button>
          </div>
        </footer>
      )}
    </section>
  );
}
