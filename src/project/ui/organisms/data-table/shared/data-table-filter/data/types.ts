import type { DataTextProps } from "../../../data/types";

export type DataTableFilterProps = {
  columns: DataTextProps[];
  filterOptions?: string[];
  searchQuery?: string;
  categoryFilter?: string;
  hiddenColumns?: number[];
  addLabel?: string;
  onSearchChange?: (value: string) => void;
  onCategoryChange?: (value: string) => void;
  onToggleColumn?: (index: number) => void;
  onReset?: () => void;
  onAdd?: () => void;
};
