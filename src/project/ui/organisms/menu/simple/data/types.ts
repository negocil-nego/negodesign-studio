import type { ButtonProps } from "../../../../atoms/button/variant/default/type/types"
import type { ImageProps } from "../../../../atoms/image/data/types"
import type { MenuTextProps } from "../../shared/menu/menu-text/data/types"
import type { SubmenuSidebarGridProps } from "../../shared/submenu/grid/sidebar/data/types"
import type { SubmenuGridTitleProps } from "../../shared/submenu/grid/title/data/types"

export type MenuTypeProps = MenuTextProps | SubmenuGridTitleProps | SubmenuSidebarGridProps

export type MenuProps = {
    logo?: ImageProps
    menus?: MenuTypeProps[]
    buttonsProps?: ButtonProps[]
    defaultOpenSubmenu?: number | null
}
