import type { TextProps } from "../../../../../../atoms/text/data/types"

export type MenuTitleProps = {
    href?: string
    variant?: "item" | "featured"
    title?: TextProps
    description?: TextProps
    link?: TextProps
}
