import type { MenuTitleProps } from "../../../../menu/menu-title/data/types"
import type { SubmenuTitleProps } from "../../../simple/title/data/types"

export type SubmenuGridTitleProps = {
    label?: string
    title?: MenuTitleProps
    items?: SubmenuTitleProps[]
}
