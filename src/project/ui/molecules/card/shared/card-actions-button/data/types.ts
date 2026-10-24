import type { IconName } from "../../../../../atoms/icon/data/types"

export type CardActionButton = {
    icon?: IconName
    label: string
    onClick?: () => void
}

export type CardActionsButtonProps = {
    actions: CardActionButton[]
}
