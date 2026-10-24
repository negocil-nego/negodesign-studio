// @ts-nocheck
import type { CardProfileProps } from "../data/types";
import CardProfile01 from "./card-01/CardProfile01";
import CardProfile02 from "./card-02/CardProfile02";

export default function CardProfile({ variant = "01", ...restProps }: CardProfileProps) {
    if (variant === "01") return <CardProfile01 {...restProps} />;
    if (variant === "02") return <CardProfile02 {...restProps} />;
    return null;
}
