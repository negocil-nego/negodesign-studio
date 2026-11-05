export type DataTableVariant = "01" | "02" | "03" | "04" | "05";

export type DataTableColumn = {
  label: string;
  key: string;
  sortable?: boolean;
};

export type DataTableRow = {
  id: string;
  cells: string[];
  avatar?: string;
  badge?: string;
};

export type DataTableProps = {
  variant?: DataTableVariant;
  title?: string;
  columns?: DataTableColumn[];
  rows?: DataTableRow[];
  className?: string;
};
