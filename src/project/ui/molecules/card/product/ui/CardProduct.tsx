// @ts-nocheck
import type { CardProductProps } from "../data/types";
import CardProduct01 from "./card-01/CardProduct01";
import CardProduct02 from "./card-02/CardProduct02";

export default function CardProduct({ variant = "01", ...restProps }: CardProductProps) {
    if (variant === "01") return <CardProduct01 {...restProps} />;
    if (variant === "02") return <CardProduct02 {...restProps} />;
    return null;
}
