import type { IconName } from "../../../../../atoms/icon/data/types"

export type CardTag = {
    icon?: IconName
    title: string
    description?: string
}
export type CardTagsProps = {
    tags: CardTag[]
    className?: string
    itemClassName?: string
    titleClassName?: string
    descriptionClassName?: string
}
