import type { TextProps } from "../../../atoms/text/data/types";

export type DataTableVariant = "01" | "02" | "03" | "04" | "05";

export type DataTableCellType =
  | "text"
  | "badge"
  | "dropdown"
  | "email"
  | "toggle"
  | "actions";

export type DataTableBadgeVariant =
  | "primary"
  | "secondary"
  | "success"
  | "danger"
  | "warning"
  | "info"
  | "neutral";

export type DataTableActionItem = {
  label: string;
  value?: string;
  icon?: string;
  variant?: "default" | "danger";
};

export type DataTextProps = TextProps & {
  /** Define se a coluna ou célula será renderizada como texto, badge, dropdown, email, toggle ou botões de acção. */
  type?: DataTableCellType;
  /** Alias opcional para `type` (ex.: `component: "dropdown"` ou `component: "badge"`). */
  component?: DataTableCellType;
  /** Variante semântica quando a célula é um `badge`. */
  badgeVariant?: DataTableBadgeVariant;
  /** Apresenta indicador circular (dot) dentro do badge. */
  dot?: boolean;
  /** Lista de opções de acção quando a célula/coluna é um `dropdown`. */
  actions?: (string | DataTableActionItem)[];
  /** Estado booleano para células do tipo `toggle`. */
  checked?: boolean;
  /** Subtítulo opcional exibido por baixo do texto principal. */
  subtitle?: string;
  /** URL de avatar opcional exibido ao lado do texto principal. */
  avatar?: string;
};

export type DataTableProps = {
  variant?: DataTableVariant;
  title?: string;
  /** Lista plana de cabeçalhos de coluna com suporte a `DataTextProps`. */
  columns?: DataTextProps[];
  /** Lista plana de células — agrupadas internamente por `columns.length`. */
  rows?: DataTextProps[];
  className?: string;
  /** Placeholder do campo de pesquisa da barra de filtros. */
  searchPlaceholder?: string;
  /** Índice da coluna usada pelo select e pelo filtro (por omissão 0). */
  filterColumn?: number;
  /** Rótulo da opção "todas" do select de filtro. */
  filterAllLabel?: string;
  /** Mostra a barra de filtros (pesquisa, select, filtro e colunas). */
  showToolbar?: boolean;
  /** Acções apresentadas no menu "⋮" da barra de ferramentas. */
  moreActions?: string[];
  /** Acções apresentadas no dropdown da coluna de acção de cada linha. */
  actions?: (string | DataTableActionItem)[];
  /** Rótulo do cabeçalho da coluna de acções quando adicionada via `actions` / `showActionColumn`. */
  actionColumnLabel?: string;
  /** Mostra automaticamente uma coluna final de acções em dropdown. */
  showActionColumn?: boolean;
  /** Callback disparado ao seleccionar uma acção no dropdown de uma linha. */
  onRowAction?: (action: string, rowIndex: number, row: DataTextProps[]) => void;
  /** Rótulo do botão primário de criação na barra superior (ex.: "+ Add User"). */
  addLabel?: string;
  /** Callback disparado ao clicar no botão de criação. */
  onAdd?: () => void;
};
