import type { DataTableProps } from "../../../organisms/data-table/data/types";
import type { ModalFormProps } from "../../../organisms/modal/form/data/types";
import type { MenuProps } from "../../../organisms/menu/simple/data/types";
import type { SidebarProps } from "../../../organisms/sidebar/data/types";

export interface AdminProps {
    datatableProps?: DataTableProps
    modalFormProps?: ModalFormProps
    sidebarProps?: SidebarProps
    menuProps?: MenuProps
}