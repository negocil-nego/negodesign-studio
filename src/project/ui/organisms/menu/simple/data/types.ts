import type { ButtonProps } from "../../../../atoms/button/variant/default/type/types"
import type { ImageProps } from "../../../../atoms/image/data/types"
import type { MenuTextProps } from "../../shared/menu/menu-text/data/types"

export type MenuProps = {
    logo?: ImageProps
    menus?: MenuTextProps[]
    buttonsProps?: ButtonProps[]
}