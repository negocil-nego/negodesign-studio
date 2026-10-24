// @ts-nocheck
import Button from "../../../../../atoms/button/variant/default/ui/Button";
import type { CardButtonProps } from "../data/types";

export default function CardButton({ variant = "solid", ...restProps }: CardButtonProps) {
    return <Button variant={variant} {...restProps} />;
}
