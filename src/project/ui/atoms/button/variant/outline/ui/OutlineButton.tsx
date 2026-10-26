// @ts-nocheck
import { StyleComponent } from "../../../../../../data/style.component";
import ButtonBase from "../../../shared/base/ui/ButtonBase";
import type { OutlineButtonProps } from "../data/types";

export default function OutlineButton(props: OutlineButtonProps) {
    return <ButtonBase className={StyleComponent.button.outline.default} {...props} />;
}
