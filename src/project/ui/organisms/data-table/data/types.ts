import type { TextProps } from "../../../atoms/text/data/types";

export type DataTableVariant = "01" | "02" | "03" | "04" | "05";

export type DataTableProps = {
  variant?: DataTableVariant;
  title?: string;
  /** Flat list of column headers. */
  columns?: TextProps[];
  /** Flat list of all cell values — chunked by columns.length inside the component. */
  rows?: TextProps[];
  className?: string;
};
