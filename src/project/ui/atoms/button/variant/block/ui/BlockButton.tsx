// @ts-nocheck
import { StyleComponent } from "../../../../../../data/style.component";
import ButtonBase from "../../../shared/base/ui/ButtonBase";
import type { BlockButtonProps } from "../data/types";

export default function BlockButton({
  variant = "primary",
  className = "",
  ...restProps
}: BlockButtonProps) {
  const variantClass =
    variant === "outline"
      ? `${StyleComponent.button.outline.default} btn-lg btn-block`
      : variant === "secondary"
        ? `${StyleComponent.button.default} btn-secondary btn-lg btn-block`
        : StyleComponent.button.block.default;

  return <ButtonBase className={`${variantClass} ${className}`.trim()} {...restProps} />;
}
