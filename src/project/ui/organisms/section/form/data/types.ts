import type { ButtonProps } from "../../../../atoms/button/variant/default/type/types"
import type { TextProps } from "../../../../atoms/text/data/types"
import type { FormProps } from "../../../../molecules/form/data/types"

export type SectionFormProps = {
    titleProps?: TextProps
    subtitleProps?: TextProps
    descriptionProps?: TextProps
    buttonSubmitProps?: ButtonProps
    buttonsLoginProps?: ButtonProps[]
    formProps?: FormProps
}