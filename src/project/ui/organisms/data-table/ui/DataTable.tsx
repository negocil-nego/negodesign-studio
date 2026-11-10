import { useMemo, useState } from "react";
import Icon from "../../../atoms/icon/ui/Icon";
import Text from "../../../atoms/text/ui/Text";
import { StyleComponent } from "../../../../data/style.component";
import { computeDataTableView, DATA_TABLE_ROW_ACTIONS, getCellKind, toggleNumberInList } from "../../../../hooks/data-table.hook";
import DataTableFilter from "../shared/data-table-filter/ui/DataTableFilter";
import type { DataTableProps, DataTableVariant } from "../data/types";

export default function DataTable({ variant = "01", title, columns, rows, className = "" }: DataTableProps) {
  const S = StyleComponent.dataTable;
  const [searchQuery, setSearchQuery] = useState("");
  const [categoryFilter, setCategoryFilter] = useState("all");
  const [sortAsc, setSortAsc] = useState(true);
  const [selected, setSelected] = useState<number[]>([]);
  const [menuRow, setMenuRow] = useState<number | null>(null);
  const [hiddenColumns, setHiddenColumns] = useState<number[]>([]);

  const view = useMemo(
    () => computeDataTableView({ variant: variant as DataTableVariant, columns, rows, searchQuery, categoryFilter, sortAsc, hiddenColumns }),
    [variant, columns, rows, searchQuery, categoryFilter, sortAsc, hiddenColumns],
  );

  return (
    <section aria-label={title ?? `Data table ${variant}`} className={`${S.root} ${className}`}>
      {title && <h3 className={S.title}>{title}</h3>}
      <DataTableFilter
        columns={view.columns}
        filterOptions={view.filterOptions}
        searchQuery={searchQuery}
        categoryFilter={categoryFilter}
        hiddenColumns={hiddenColumns}
        onSearchChange={setSearchQuery}
        onCategoryChange={setCategoryFilter}
        onToggleColumn={(ci) => setHiddenColumns((p) => toggleNumberInList(p, ci, true))}
        onReset={() => { setSearchQuery(""); setCategoryFilter("all"); setHiddenColumns([]); }}
      />
      <div className={S.viewport}>
        <table className={`${S.table} ${variant === "02" ? S.tableCompact : ""}`}>
          <thead className={S.head}>
            <tr>
              {view.showCheckbox && <th className={S.checkboxCell}><input type="checkbox" className={S.checkbox} aria-label="Select all" /></th>}
              {view.columns.map((col, ci) => !hiddenColumns.includes(ci) && (
                <th key={ci} className={`${S.headCell} ${col.className ?? ""}`}>
                  <button className={S.headCellButton} onClick={() => setSortAsc(!sortAsc)}>
                    <Text text={col.text} /><span aria-hidden className={S.headCellSortIcon}>↕</span>
                  </button>
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {view.visibleRows.map((row, ri) => (
              <tr key={ri} className={S.row}>
                {view.showCheckbox && (
                  <td className="px-3 py-2"><input type="checkbox" className={S.checkbox} checked={selected.includes(ri)} onChange={() => setSelected((p) => toggleNumberInList(p, ri))} /></td>
                )}
                {view.columns.map((_, ci) => !hiddenColumns.includes(ci) && (() => {
                  const cell = row[ci] ?? { text: "" };
                  const kind = getCellKind(variant as DataTableVariant, ci);
                  return (
                    <td key={ci} className={`${S.cell} ${variant === "02" ? S.cellCompact : ""}`}>
                      {kind === "email" ? <a href={`mailto:${cell.text}`} className="text-blue-400 underline underline-offset-2"><Text {...cell} /></a>
                        : (kind === "invoice" || kind === "edit") ? <span className="inline-flex gap-1"><button className={S.actionButton}>{kind === "invoice" ? "Invoice" : "Edit"}</button><button className={S.actionButton}>{kind === "invoice" ? "Ledger" : "Delete"}</button></span>
                        : kind === "menu" ? (
                          <span className="relative inline-flex gap-1">
                            <button className={S.actionButton}><Icon name="Phone" /></button>
                            <button className={S.actionButton}><Icon name="FolderAdd" /></button>
                            <button className={S.actionButton} onClick={() => setMenuRow(menuRow === ri ? null : ri)}><Icon name="MoreHorizontal" /></button>
                            {menuRow === ri && <span className={S.rowMenu}>{DATA_TABLE_ROW_ACTIONS.map((a) => <button key={a} className={S.rowMenuItem}>{a}</button>)}</span>}
                          </span>
                        ) : <span className={variant === "02" && ci === 0 ? "font-semibold text-zinc-100" : "text-zinc-200"}><Text {...cell} /></span>}
                    </td>
                  );
                })())}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      {view.showFooter && (
        <footer className={S.footer}>
          <span>{view.pageLabel}</span>
          <div className={S.footerGroup}>
            <label className="flex items-center gap-2">Rows per page <select className={S.footerSelect}><option>10</option><option>20</option></select></label>
            <button className={S.footerButton}>‹</button><button className={S.footerButton}>›</button>
          </div>
        </footer>
      )}
    </section>
  );
}
