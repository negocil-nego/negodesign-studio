import { useMemo, useState } from "react";
import Icon from "../../../atoms/icon/ui/Icon";
import Text from "../../../atoms/text/ui/Text";
import { StyleComponent } from "../../../../data/style.component";
import {
  computeDataTableView,
  getCellKind,
  normalizeActionItems,
  resolveBadgeVariantClass,
  toggleNumberInList,
} from "../../../../hooks/data-table.hook";
import DataTableFilter from "../shared/data-table-filter/ui/DataTableFilter";
import type { DataTableProps, DataTableVariant, DataTextProps } from "../data/types";

export default function DataTable({
  variant = "01",
  title,
  columns,
  rows,
  className = "",
  actions,
  actionColumnLabel,
  showActionColumn,
  onRowAction,
  addLabel,
  onAdd,
}: DataTableProps) {
  const S = StyleComponent.dataTable;
  const [searchQuery, setSearchQuery] = useState("");
  const [categoryFilter, setCategoryFilter] = useState("all");
  const [sortAsc, setSortAsc] = useState(true);
  const [selected, setSelected] = useState<number[]>([]);
  const [menuCellKey, setMenuCellKey] = useState<string | null>(null);
  const [hiddenColumns, setHiddenColumns] = useState<number[]>([]);
  const [toggleStates, setToggleStates] = useState<Record<string, boolean>>({});

  const view = useMemo(
    () =>
      computeDataTableView({
        variant: variant as DataTableVariant,
        columns,
        rows,
        actions,
        actionColumnLabel,
        showActionColumn,
        searchQuery,
        categoryFilter,
        sortAsc,
        hiddenColumns,
      }),
    [variant, columns, rows, actions, actionColumnLabel, showActionColumn, searchQuery, categoryFilter, sortAsc, hiddenColumns],
  );

  const handleActionSelect = (actionVal: string, ri: number, row: DataTextProps[]) => {
    setMenuCellKey(null);
    onRowAction?.(actionVal, ri, row);
  };

  return (
    <>
      {menuCellKey && <div className={S.overlay} role="presentation" onClick={() => setMenuCellKey(null)} />}
      <section aria-label={title ?? `Data table ${variant}`} className={`${S.root} ${className}`}>
        {title && <h3 className={S.title}>{title}</h3>}
        <DataTableFilter
          columns={view.columns}
          filterOptions={view.filterOptions}
          searchQuery={searchQuery}
          categoryFilter={categoryFilter}
          hiddenColumns={hiddenColumns}
          addLabel={addLabel}
          onAdd={onAdd}
          onSearchChange={setSearchQuery}
          onCategoryChange={setCategoryFilter}
          onToggleColumn={(ci) => setHiddenColumns((p) => toggleNumberInList(p, ci, true))}
          onReset={() => { setSearchQuery(""); setCategoryFilter("all"); setHiddenColumns([]); }}
        />
        {selected.length > 0 && (
          <div className={S.selectionBar}>
            <span>{selected.length} selected</span>
            <button type="button" className="text-xs font-medium underline" onClick={() => setSelected([])}>Clear selection</button>
          </div>
        )}
        <div className={S.viewport}>
          <table className={`${S.table} ${variant === "02" ? S.tableCompact : ""}`}>
            <thead className={S.head}>
              <tr>
                {view.showCheckbox && (
                  <th className={S.checkboxCell}>
                    <input
                      type="checkbox"
                      className={S.checkbox}
                      aria-label="Select all"
                      checked={view.visibleRows.length > 0 && selected.length === view.visibleRows.length}
                      onChange={(e) => setSelected(e.target.checked ? view.visibleRows.map((_, i) => i) : [])}
                    />
                  </th>
                )}
                {view.columns.map((col, ci) => !hiddenColumns.includes(ci) && (
                  <th key={ci} className={`${S.headCell} ${col.className ?? ""}`}>
                    <button type="button" className={S.headCellButton} onClick={() => setSortAsc(!sortAsc)}>
                      <Text text={col.text} /><span aria-hidden className={S.headCellSortIcon}>↕</span>
                    </button>
                  </th>
                ))}
              </tr>
            </thead>
            <tbody className={S.tbody}>
              {view.visibleRows.map((row, ri) => (
                <tr key={ri} className={`${S.row} ${selected.includes(ri) ? S.trSelected : ""}`}>
                  {view.showCheckbox && (
                    <td className="px-3 py-2">
                      <input
                        type="checkbox"
                        className={S.checkbox}
                        checked={selected.includes(ri)}
                        onChange={() => setSelected((p) => toggleNumberInList(p, ri))}
                      />
                    </td>
                  )}
                  {view.columns.map((col, ci) => !hiddenColumns.includes(ci) && (() => {
                    const cell = row[ci] ?? { text: "" };
                    const kind = getCellKind(variant as DataTableVariant, ci, cell, col);
                    const cellId = `${ri}-${ci}`;
                    const isChecked = toggleStates[cellId] ?? cell.checked ?? false;
                    const rowActions = normalizeActionItems(cell, col, actions);

                    return (
                      <td key={ci} className={`${S.cell} ${variant === "02" ? S.cellCompact : ""}`}>
                        {kind === "email" ? (
                          <a href={`mailto:${cell.text}`} className="text-blue-400 underline underline-offset-2"><Text {...cell} /></a>
                        ) : kind === "badge" ? (
                          <span className={cell.className ? cell.className : `${S.badge.base} ${resolveBadgeVariantClass(cell.badgeVariant, cell.text, S.badge)}`}>
                            {cell.dot && <span className={S.badge.dot} aria-hidden />}
                            <span>{cell.text}</span>
                          </span>
                        ) : kind === "toggle" ? (
                          <button
                            type="button"
                            role="switch"
                            aria-checked={isChecked}
                            aria-label={col.text || "Toggle"}
                            onClick={() => setToggleStates((prev) => ({ ...prev, [cellId]: !isChecked }))}
                            className={`relative inline-flex h-5 w-9 items-center rounded-full transition-colors ${isChecked ? "bg-blue-600" : "bg-zinc-700"}`}
                          >
                            <span className={`inline-block size-3.5 transform rounded-full bg-white transition-transform ${isChecked ? "translate-x-4" : "translate-x-1"}`} />
                          </button>
                        ) : (kind === "invoice" || kind === "edit") ? (
                          <span className="inline-flex gap-1">
                            <button type="button" className={S.actionButton} onClick={() => handleActionSelect(kind === "invoice" ? "invoice" : "edit", ri, row)}>
                              {kind === "invoice" ? "Invoice" : "Edit"}
                            </button>
                            <button type="button" className={S.actionButton} onClick={() => handleActionSelect(kind === "invoice" ? "ledger" : "delete", ri, row)}>
                              {kind === "invoice" ? "Ledger" : "Delete"}
                            </button>
                          </span>
                        ) : kind === "dropdown" ? (
                          <div className="relative inline-flex items-center gap-1">
                            <button
                              type="button"
                              className={S.menuButton}
                              aria-label="Row actions"
                              aria-expanded={menuCellKey === cellId}
                              onClick={() => setMenuCellKey(menuCellKey === cellId ? null : cellId)}
                            >
                              {cell.text ? <span className="px-1.5 text-xs">{cell.text}</span> : <Icon name="MoreHorizontal" />}
                            </button>
                            {menuCellKey === cellId && (
                              <div className={S.rowMenu} role="menu">
                                {rowActions.map((act) => (
                                  <button
                                    key={act.label}
                                    type="button"
                                    role="menuitem"
                                    className={act.variant === "danger" ? S.rowMenuDanger : S.rowMenuItem}
                                    onClick={() => handleActionSelect(act.value ?? act.label, ri, row)}
                                  >
                                    {act.icon && <Icon name={act.icon} />}
                                    <span>{act.label}</span>
                                  </button>
                                ))}
                              </div>
                            )}
                          </div>
                        ) : (
                          <div className="flex items-center gap-2.5">
                            {cell.avatar && <img src={cell.avatar} alt={cell.text} className="size-7 rounded-full object-cover" />}
                            <div>
                              <span className={variant === "02" && ci === 0 ? "font-semibold text-zinc-100" : "text-zinc-200"}>
                                <Text {...cell} />
                              </span>
                              {cell.subtitle && <span className="block text-[11px] text-zinc-400">{cell.subtitle}</span>}
                            </div>
                          </div>
                        )}
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
              <label className="flex items-center gap-2">Rows per page <select className={S.footerSelect}><option>10</option><option>20</option><option>50</option></select></label>
              <button type="button" className={S.footerButton}>‹</button>
              <button type="button" className={S.footerButton}>›</button>
            </div>
          </footer>
        )}
      </section>
    </>
  );
}
