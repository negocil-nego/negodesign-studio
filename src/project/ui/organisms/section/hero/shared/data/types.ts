import type { TextProps } from "../../../../../atoms/text/data/types"

export type SectionTextProps = {
    titleProps?: TextProps
    subtitleProps?: TextProps
    descriptionProps?: TextProps
    imageUrl?: string
    imageAlt?: string
    layout?: "simple" | "dual"
}

export type SectionHereProps = {
    backgroundMediaUrl?: string
    backgroundColor?: string
    textSectionProps?: SectionTextProps
}
