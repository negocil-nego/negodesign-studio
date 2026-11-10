import { dataTableExampleByVariant } from "../ui/organisms/data-table/data/examples";
import type { DataTableVariant } from "../ui/organisms/data-table/data/types";
import type { TextProps } from "../ui/atoms/text/data/types";

export type FilterPanelType = "filter" | "columns" | "more" | null;

export type DataTableStateInput = {
  variant?: DataTableVariant;
  columns?: TextProps[];
  rows?: TextProps[];
  searchQuery: string;
  categoryFilter: string;
  sortAsc: boolean;
  hiddenColumns: number[];
};

export const DATA_TABLE_ROW_ACTIONS = ["Details", "Offer a deal", "Add to my list", "Delete"] as const;

export function computeDataTableView(input: DataTableStateInput) {
  const variant = input.variant ?? "01";
  const example = dataTableExampleByVariant[variant] ?? dataTableExampleByVariant["01"];
  const columns = input.columns ?? example.columns;
  const flat = input.rows ?? example.rows;
  const colCount = columns.length || 1;

  const allRows: TextProps[][] = [];
  for (let i = 0; i < flat.length; i += colCount) {
    allRows.push(flat.slice(i, i + colCount));
  }

  const filterColIdx = columns.length > 1 ? 1 : 0;
  const optionsSet = new Set<string>();
  for (const row of allRows) {
    const val = row[filterColIdx]?.text?.trim();
    if (val) optionsSet.add(val);
  }
  const filterOptions = [...optionsSet];

  let visibleRows = input.sortAsc ? [...allRows] : [...allRows].reverse();
  if (input.categoryFilter !== "all") {
    visibleRows = visibleRows.filter((r) => r[filterColIdx]?.text === input.categoryFilter);
  }
  if (input.searchQuery.trim()) {
    const q = input.searchQuery.toLowerCase();
    visibleRows = visibleRows.filter((r) => r.some((c) => c.text.toLowerCase().includes(q)));
  }

  return {
    columns,
    filterOptions,
    visibleRows,
    showCheckbox: variant !== "01",
    showFooter: variant !== "01" && variant !== "05",
    pageLabel: variant === "02" ? "Page 1 of 13" : "Page 1 of 8",
  };
}

export function toggleNumberInList(list: number[], value: number, lockZero = false): number[] {
  if (lockZero && value === 0) return list;
  return list.includes(value) ? list.filter((x) => x !== value) : [...list, value];
}

export function togglePanelState(current: FilterPanelType, target: Exclude<FilterPanelType, null>): FilterPanelType {
  return current === target ? null : target;
}

export function getCellKind(variant: DataTableVariant, colIndex: number): "email" | "invoice" | "menu" | "edit" | "text" {
  if (variant === "03" && colIndex === 1) return "email";
  if (variant === "04" && colIndex === 7) return "invoice";
  if (variant === "01" && colIndex === 6) return "menu";
  if (variant === "03" && colIndex === 6) return "edit";
  return "text";
}
