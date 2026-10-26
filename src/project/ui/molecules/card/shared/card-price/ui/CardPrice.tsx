// @ts-nocheck
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardPriceProps } from "../data/types";

export default function CardPrice({ number }: CardPriceProps) {
    return (
        <div className={StyleComponent.card.price.root} id="card-price">
            <div className={StyleComponent.card.price.number}>{number}</div>
        </div>
    );
}
