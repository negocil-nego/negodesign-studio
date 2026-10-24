// @ts-nocheck
import type { CardPriceProps } from "../data/types";

export default function CardPrice({ number }: CardPriceProps) {
    return (
        <div className="card-price" id="card-price">
            <div className="card-price-number">{number}</div>
        </div>
    );
}
