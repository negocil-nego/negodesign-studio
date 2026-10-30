import type { MenuTitleProps } from "../../../../menu/menu-title/data/types"
import type { SubmenuTitleProps } from "../../../simple/title/data/types"

export type SubmenuSidebarGridProps = {
    type: "sidebar-grid"
    title?: MenuTitleProps
    items?: SubmenuTitleProps[]
    defaultSelected?: number
}
