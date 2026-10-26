// @ts-nocheck
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardPhoneProps } from "../data/types";

export default function CardPhone({ phone }: CardPhoneProps) {
    return (
        <div className={StyleComponent.card.phone.root} id="card-phone">
            <div className={StyleComponent.card.phone.number}>{phone}</div>
        </div>
    );
}
