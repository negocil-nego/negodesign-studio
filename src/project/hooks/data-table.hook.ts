import { dataTableExampleByVariant } from "../ui/organisms/data-table/data/examples";
import type {
  DataTableActionItem,
  DataTableBadgeVariant,
  DataTableCellType,
  DataTableVariant,
  DataTextProps,
} from "../ui/organisms/data-table/data/types";

export type FilterPanelType = "filter" | "columns" | "more" | null;

export type DataTableStateInput = {
  variant?: DataTableVariant;
  columns?: DataTextProps[];
  rows?: DataTextProps[];
  actions?: (string | DataTableActionItem)[];
  actionColumnLabel?: string;
  showActionColumn?: boolean;
  searchQuery: string;
  categoryFilter: string;
  sortAsc: boolean;
  hiddenColumns: number[];
};

export const DATA_TABLE_ROW_ACTIONS = ["Edit", "Details", "Duplicate", "Delete"] as const;

export function normalizeActionItems(
  cell?: DataTextProps,
  col?: DataTextProps,
  tableActions?: (string | DataTableActionItem)[],
): DataTableActionItem[] {
  const raw = cell?.actions ?? col?.actions ?? tableActions ?? [...DATA_TABLE_ROW_ACTIONS];
  return raw.map((item) => {
    if (typeof item === "string") {
      const lower = item.toLowerCase();
      return {
        label: item,
        value: lower,
        variant: lower.includes("delete") || lower.includes("eliminar") || lower.includes("remover") ? "danger" : "default",
      };
    }
    return {
      label: item.label,
      value: item.value ?? item.label.toLowerCase(),
      icon: item.icon,
      variant: item.variant ?? "default",
    };
  });
}

export function computeDataTableView(input: DataTableStateInput) {
  const variant = input.variant ?? "01";
  const example = dataTableExampleByVariant[variant] ?? dataTableExampleByVariant["01"];
  const baseColumns = input.columns ?? example.columns;
  const flat = input.rows ?? example.rows;
  const baseColCount = baseColumns.length || 1;

  const hasExplicitDropdownColumn = baseColumns.some(
    (c) => (c.type ?? c.component) === "dropdown" || (c.type ?? c.component) === "actions",
  );
  const appendActionColumn =
    !hasExplicitDropdownColumn &&
    (input.showActionColumn === true || (input.actions && input.actions.length > 0));

  const columns: DataTextProps[] = appendActionColumn
    ? [
        ...baseColumns,
        {
          text: input.actionColumnLabel ?? "Actions",
          type: "dropdown",
          actions: input.actions,
        },
      ]
    : baseColumns;

  const allRows: DataTextProps[][] = [];
  for (let i = 0; i < flat.length; i += baseColCount) {
    const chunk = flat.slice(i, i + baseColCount);
    if (appendActionColumn) {
      chunk.push({
        text: "",
        type: "dropdown",
        actions: input.actions,
      });
    }
    allRows.push(chunk);
  }

  const filterColIdx = baseColumns.length > 1 ? 1 : 0;
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
    visibleRows = visibleRows.filter((r) =>
      r.some((c) => (c.text ?? "").toLowerCase().includes(q) || (c.subtitle ?? "").toLowerCase().includes(q)),
    );
  }

  return {
    columns,
    filterOptions,
    visibleRows,
    showCheckbox: variant !== "01" || appendActionColumn || hasExplicitDropdownColumn,
    showFooter: variant !== "01" && variant !== "05" || appendActionColumn || hasExplicitDropdownColumn,
    pageLabel: variant === "02" ? "Page 1 of 13" : `1-${visibleRows.length} of ${allRows.length}`,
  };
}

export function toggleNumberInList(list: number[], value: number, lockZero = false): number[] {
  if (lockZero && value === 0) return list;
  return list.includes(value) ? list.filter((x) => x !== value) : [...list, value];
}

export function togglePanelState(current: FilterPanelType, target: Exclude<FilterPanelType, null>): FilterPanelType {
  return current === target ? null : target;
}

export function getCellKind(
  variant: DataTableVariant,
  colIndex: number,
  cell?: DataTextProps,
  col?: DataTextProps,
): "email" | "invoice" | "dropdown" | "edit" | "badge" | "toggle" | "text" {
  const explicit: DataTableCellType | undefined =
    cell?.type ?? cell?.component ?? col?.type ?? col?.component;

  if (explicit === "dropdown" || explicit === "actions") return "dropdown";
  if (explicit === "badge") return "badge";
  if (explicit === "email") return "email";
  if (explicit === "toggle") return "toggle";
  if (cell?.badgeVariant) return "badge";
  if (cell?.actions && cell.actions.length > 0) return "dropdown";

  if (variant === "03" && colIndex === 1) return "email";
  if (variant === "04" && colIndex === 7) return "invoice";
  if (variant === "01" && colIndex === 6) return "dropdown";
  if (variant === "03" && colIndex === 6) return "edit";
  return "text";
}

export function resolveBadgeVariantClass(
  badgeVariant: DataTableBadgeVariant | undefined,
  text: string,
  badgeStyles: Record<string, string>,
): string {
  if (badgeVariant && badgeStyles[badgeVariant]) {
    return badgeStyles[badgeVariant];
  }
  const lower = (text ?? "").toLowerCase();
  if (lower.includes("active") || lower.includes("full time") || lower.includes("enabled") || lower.includes("medium")) {
    return badgeStyles.success ?? "";
  }
  if (lower.includes("part time") || lower.includes("high") || lower.includes("pending")) {
    return badgeStyles.warning ?? "";
  }
  if (lower.includes("inactive") || lower.includes("failed") || lower.includes("urgent")) {
    return badgeStyles.danger ?? "";
  }
  if (lower.includes("admin") || lower.includes("moderator")) {
    return badgeStyles.primary ?? "";
  }
  return badgeStyles.neutral ?? "";
}
