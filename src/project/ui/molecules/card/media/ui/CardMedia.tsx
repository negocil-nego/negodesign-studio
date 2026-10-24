// @ts-nocheck
import type { CardMediaProps } from "../data/types";
import CardMedia01 from "./card-01/CardMedia01";
import CardMedia02 from "./card-02/CardMedia02";

export default function CardMedia({ variant = "01", ...restProps }: CardMediaProps) {
    if (variant === "01") return <CardMedia01 {...restProps} />;
    if (variant === "02") return <CardMedia02 {...restProps} />;
    return null;
}
