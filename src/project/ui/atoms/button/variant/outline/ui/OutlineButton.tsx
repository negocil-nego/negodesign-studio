// @ts-nocheck
import ButtonBase from "../../../shared/base/ui/ButtonBase";
import type { OutlineButtonProps } from "../data/types";

export default function OutlineButton(props: OutlineButtonProps) {
    return <ButtonBase className={"w-full py-2 px-4 rounded-md text-center border border-primary/40 text-primary-foreground"} {...props} />;
}
