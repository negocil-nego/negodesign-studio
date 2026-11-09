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
  /** Placeholder do campo de pesquisa da barra de filtros. */
  searchPlaceholder?: string;
  /** Índice da coluna usada pelo select e pelo filtro (por omissão 0). */
  filterColumn?: number;
  /** Rótulo da opção "todas" do select de filtro. */
  filterAllLabel?: string;
  /** Mostra a barra de filtros (pesquisa, select, filtro e colunas). */
  showToolbar?: boolean;
  /** Acções apresentadas no menu "⋮". */
  moreActions?: string[];
};
