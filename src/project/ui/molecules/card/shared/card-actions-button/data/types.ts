import type { IconName } from "../../../../../atoms/icon/data/types"

export type CardActionButton = {
    icon?: IconName
    label?: string
    onClick?: () => void
}

/**
 * variant:
 *  - "default"      → outline buttons with optional icon + label (original behaviour)
 *  - "icon-outline" → icon-only round outline buttons (e.g. Edit Profile)
 *  - "mixed"        → first action renders as solid pill (Follow), rest as icon-only outline (Message)
 */
export type CardActionsButtonVariant = "default" | "icon-outline" | "mixed"

export type CardActionsButtonProps = {
    actions: CardActionButton[]
    variant?: CardActionsButtonVariant
}
