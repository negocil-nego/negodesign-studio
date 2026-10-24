// @ts-nocheck
import ButtonBase from "../../../shared/base/ui/ButtonBase";
import type { SolidButtonProps } from "../data/types";

export default function SolidButton(props: SolidButtonProps) {
    return <ButtonBase className={"w-full py-2 px-4 rounded-md text-center bg-blue-900 hover:bg-primary/80 text-white"} {...props} />;
}
