import type { TextProps } from "../../../../atoms/text/data/types"
import type { MenuProps } from "../../../../organisms/menu/simple/data/types"

export type PageHeroSimpleProps = {
    titleProps?: TextProps
    subtitleProps?: TextProps
    descriptionProps?: TextProps
    menuProps?: MenuProps
    backgroundMediaUrl?: string
    backgroundColor?: string
}
