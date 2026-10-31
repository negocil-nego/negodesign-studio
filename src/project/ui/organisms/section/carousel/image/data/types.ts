import type { TextProps } from "../../../../../atoms/text/data/types"

export type SectionImageCarouselItem = {
    title?: TextProps
    description?: TextProps
    backgroundImage?: string
}

export type SectionImageCarouselProps = {
    items?: SectionImageCarouselItem[]
    className?: string
    /** Gradiente CSS (ex.: linear-gradient(...)) ou classes CSS utilitárias para o fundo. */
    backgroundColor?: string
}
