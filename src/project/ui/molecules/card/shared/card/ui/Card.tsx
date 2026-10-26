// @ts-nocheck
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardProps } from "../data/types";

export default function Card({ ...restProps }: CardProps) {
    return (
        <div className={StyleComponent.card.root} id="card">
            <div {...restProps} />
        </div>
    );
}
