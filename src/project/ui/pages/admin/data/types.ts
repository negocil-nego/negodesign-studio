import type { BreadcrumbProps } from "../../../molecules/breadcrumb/data/types";
import type { DataTableProps, DataTextProps } from "../../../organisms/data-table/data/types";
import type { ModalFormProps } from "../../../organisms/modal/form/data/types";
import type { MenuProps } from "../../../organisms/menu/simple/data/types";
import type { SidebarProps } from "../../../organisms/sidebar/data/types";

export interface AdminProps {
  datatableProps?: DataTableProps;
  modalFormProps?: ModalFormProps;
  sidebarProps?: SidebarProps;
  menuProps?: MenuProps;
  breadcrumb?: BreadcrumbProps;
  breadcrumbProps?: BreadcrumbProps;
  title?: string;
  addButtonLabel?: string;
  className?: string;
  onCreateClick?: () => void;
  onRowAction?: (action: string, rowIndex: number, row: DataTextProps[]) => void;
}

export type AdminCrudProps = AdminProps;
