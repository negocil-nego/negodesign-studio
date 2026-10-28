import type { ButtonProps } from "../../../../atoms/button/variant/default/type/types"
import type { TextProps } from "../../../../atoms/text/data/types"
import type { FormProps } from "../../../../molecules/form/data/types"

export type SectionFormProps = {
    isScreen?: boolean
    className?: string
    titleProps?: TextProps
    subtitleProps?: TextProps
    descriptionProps?: TextProps
    buttonSubmitProps?: ButtonProps
    buttonsLoginProps?: ButtonProps[]
    formProps?: FormProps
}