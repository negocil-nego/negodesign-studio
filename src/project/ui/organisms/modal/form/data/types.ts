import type { FormProps } from "../../../../molecules/form/data/types"

export type ModalFormProps = {
    open?: boolean
    onClose?: () => void
    title?: string
    form?: FormProps
}
