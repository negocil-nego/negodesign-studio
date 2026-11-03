import type { MenuProps } from "../../../../organisms/menu/simple/data/types"
import type { SectionHereProps } from "../../../../organisms/section/hero/shared/data/types"

export type PageHeroDualProps = {
    menuProps?: MenuProps
    leftSectionHereProps?: SectionHereProps,
    rightSectionHereProps?: SectionHereProps,
}