import type { TextProps } from "../../../../../atoms/text/data/types"

export type SectionTextProps = {
    titleProps?: TextProps
    subtitleProps?: TextProps
    descriptionProps?: TextProps
}

export type SectionHereProps = {
    backgroundMediaUrl?: string
    backgroundColor?: string
    textSectionProps?: SectionTextProps
}