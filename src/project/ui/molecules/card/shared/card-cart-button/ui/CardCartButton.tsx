// @ts-nocheck
import IconButton from "../../../../../atoms/button/shared/icon/ui/IconButton";
import type { CardCartButtonProps } from "../data/types";

export default function CardCartButton({ ...restProps }: CardCartButtonProps) {
    return <IconButton icon={{ name: "Cart" }} {...restProps} />;
}
