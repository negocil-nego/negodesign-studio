import type { Snippet } from "svelte"

export type ModalProps = {
    open?: boolean
    onClose?: () => void
    title?: string
    children?: Snippet
}
