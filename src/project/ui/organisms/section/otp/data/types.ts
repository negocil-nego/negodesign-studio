import type { ButtonProps } from "../../../../atoms/button/variant/default/type/types"
import type { InputOtpProps } from "../../../../atoms/input/children/otp/data/types"
import type { TextProps } from "../../../../atoms/text/data/types"

export type SectionOtpProps = {
    isScreen?: boolean
    layout?: "simple" | "dual"
    className?: string
    brandTitleProps?: TextProps
    brandSubtitleProps?: TextProps
    brandDescriptionProps?: TextProps
    titleProps?: TextProps
    subtitleProps?: TextProps
    descriptionProps?: TextProps
    buttonSubmitProps?: ButtonProps
    buttonsLoginProps?: ButtonProps[]
    inputOtpProps?: InputOtpProps
}
