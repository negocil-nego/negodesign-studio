import { useState } from "react";
import Icon from "../../../../../atoms/icon/ui/Icon";
import { StyleComponent } from "../../../../../../data/style.component";
import type { DataTableFilterProps } from "../data/types";

export default function DataTableFilter({
  columns = [],
  filterOptions = [],
  searchQuery = "",
  categoryFilter = "all",
  hiddenColumns = [],
  onSearchChange,
  onCategoryChange,
  onToggleColumn,
  onReset,
}: DataTableFilterProps) {
  const S = StyleComponent.dataTable;
  const [activePanel, setActivePanel] = useState<"filter" | "columns" | "more" | null>(null);

  const isHidden = (index: number) => hiddenColumns.includes(index);

  return (
    <>
      {activePanel !== null && (
        <div className={S.overlay} role="presentation" onClick={() => setActivePanel(null)} />
      )}

      <div className={S.toolbar}>
        <label className={S.search}>
          <span className={S.searchIcon}><Icon name="Search01" /></span>
          <input
            className={S.searchInput}
            type="search"
            placeholder="Search"
            value={searchQuery}
            onChange={(e) => onSearchChange?.(e.target.value)}
            aria-label="Pesquisar na tabela"
          />
        </label>

        <div className="ml-auto flex flex-wrap items-center gap-2">
          <label className={S.filterSelect}>
            <select
              className={S.filterSelectInput}
              value={categoryFilter}
              onChange={(e) => onCategoryChange?.(e.target.value)}
              aria-label="Filtrar por categoria"
            >
              <option value="all">All Campaigns</option>
              {filterOptions.map((opt) => (
                <option key={opt} value={opt}>{opt}</option>
              ))}
            </select>
            <span className={S.filterSelectIcon}><Icon name="ArrowDown01" /></span>
          </label>

          <div className="relative">
            <button
              type="button"
              className={`${S.button} ${activePanel === "filter" ? S.buttonActive : ""}`}
              onClick={() => setActivePanel(activePanel === "filter" ? null : "filter")}
              aria-expanded={activePanel === "filter"}
            >
              <Icon name="FilterHorizontal" />
              <span>Filter</span>
            </button>

            {activePanel === "filter" && (
              <div className={S.panelLeft} role="menu">
                <button
                  type="button"
                  className={S.option}
                  onClick={() => { onCategoryChange?.("all"); setActivePanel(null); }}
                >
                  <span className={S.optionLabel}>All Campaigns</span>
                </button>
                {filterOptions.map((opt) => (
                  <button
                    key={opt}
                    type="button"
                    className={S.option}
                    onClick={() => { onCategoryChange?.(opt); setActivePanel(null); }}
                  >
                    <span className={S.optionLabel}>{opt}</span>
                  </button>
                ))}
              </div>
            )}
          </div>

          <div className="relative">
            <button
              type="button"
              className={`${S.button} ${activePanel === "columns" ? S.buttonActive : ""}`}
              onClick={() => setActivePanel(activePanel === "columns" ? null : "columns")}
              aria-expanded={activePanel === "columns"}
            >
              <Icon name="LayoutThreeColumn" />
              <span>Columns</span>
            </button>

            {activePanel === "columns" && (
              <div className={S.panelRight} role="menu">
                {columns.map((col, ci) => (
                  <button
                    key={ci}
                    type="button"
                    className={`${S.option} ${ci === 0 ? S.optionDisabled : ""}`}
                    disabled={ci === 0}
                    onClick={() => ci !== 0 && onToggleColumn?.(ci)}
                    role="menuitemcheckbox"
                    aria-checked={!isHidden(ci)}
                  >
                    <input
                      type="checkbox"
                      className={S.optionInput}
                      checked={!isHidden(ci)}
                      disabled={ci === 0}
                      readOnly
                    />
                    <span className={S.optionLabel}>{col.text}</span>
                    {ci !== 0 && (
                      <span className={S.dragHandle}><Icon name="DragDropVertical" /></span>
                    )}
                  </button>
                ))}
              </div>
            )}
          </div>

          <div className="relative">
            <button
              type="button"
              className={S.iconButton}
              aria-label="Mais opções"
              onClick={() => setActivePanel(activePanel === "more" ? null : "more")}
            >
              <Icon name="MoreVertical" />
            </button>

            {activePanel === "more" && (
              <div className={S.menu} role="menu">
                <button
                  type="button"
                  className={S.menuItem}
                  onClick={() => { onReset?.(); setActivePanel(null); }}
                >
                  Reset filters
                </button>
                <button type="button" className={S.menuItem} onClick={() => setActivePanel(null)}>
                  Export CSV
                </button>
              </div>
            )}
          </div>
        </div>
      </div>
    </>
  );
}
