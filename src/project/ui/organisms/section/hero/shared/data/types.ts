import type { TextProps } from "../../../../../atoms/text/data/types"
import type { ButtonProps } from "../../../../../atoms/button/variant/default/type/types"

export type SectionTextProps = {
    titleProps?: TextProps
    subtitleProps?: TextProps
    descriptionProps?: TextProps
    imageUrl?: string
    imageAlt?: string
    buttonsProps?: ButtonProps[]
    layout?: "simple" | "dual" | "light"
}

export type SectionHereProps = {
    style?: "default" | "light"
    backgroundMediaUrl?: string
    backgroundColor?: string
    backgroundOverlayOpacity?: number
    textSectionProps?: SectionTextProps
}
