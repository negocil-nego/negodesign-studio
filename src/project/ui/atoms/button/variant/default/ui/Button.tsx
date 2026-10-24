// @ts-nocheck
import ButtonBase from "../../../shared/base/ui/ButtonBase";
import SolidButton from "../../solid/ui/SolidButton";
import OutlineButton from "../../outline/ui/OutlineButton";
import type { ButtonProps } from "../type/types";

export default function Button({ variant = "solid", ...restProps }: ButtonProps) {
    if (variant === "solid") {
        return <SolidButton {...restProps} />;
    }
    if (variant === "outline") {
        return <OutlineButton {...restProps} />;
    }
    return <ButtonBase {...restProps} />;
}
