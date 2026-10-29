import type { ButtonProps } from "../../../../atoms/button/variant/default/type/types"
import type { IconProps } from "../../../../atoms/icon/data/types"
import type { ImageProps } from "../../../../atoms/image/data/types"
import type { TextProps } from "../../../../atoms/text/data/types"

export type MenuSimpleIt = {
    icon?: IconProps
    textProps?: TextProps
}

export type MenuSimpleProps = {
    logo?: ImageProps
    menus?: MenuSimpleIt[]
    buttonsProps?: ButtonProps[]
}