import type { TextProps } from "../../../../../atoms/text/data/types";

export type DataTableFilterProps = {
  columns: TextProps[];
  filterOptions?: string[];
  searchQuery?: string;
  categoryFilter?: string;
  hiddenColumns?: number[];
  onSearchChange?: (value: string) => void;
  onCategoryChange?: (value: string) => void;
  onToggleColumn?: (index: number) => void;
  onReset?: () => void;
};
