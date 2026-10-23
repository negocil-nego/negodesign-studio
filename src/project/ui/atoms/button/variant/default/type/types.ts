import type { ButtonBaseProps } from "../../../shared/base/data/types";

export type ButtonProps = ButtonBaseProps & {
    variant: "solid" | "outline";
}