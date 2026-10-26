// @ts-nocheck
import { StyleComponent } from "../../../../../../data/style.component";
import ButtonBase from "../../../shared/base/ui/ButtonBase";
import type { SolidButtonProps } from "../data/types";

export default function SolidButton(props: SolidButtonProps) {
    return <ButtonBase className={StyleComponent.button.solid.default} {...props} />;
}
