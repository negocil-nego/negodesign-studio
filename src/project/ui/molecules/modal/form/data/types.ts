import type { FormProps } from "../../../form/data/types"

export type ModalFormProps = {
    open?: boolean
    onClose?: () => void
    title?: string
    form?: FormProps
}
