// @ts-nocheck
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardTitleProps } from "../data/types";

export default function CardTitle({ title, className }: CardTitleProps) {
    return (
        <div className={`${StyleComponent.card.title.root} ${className ?? ""}`} id="card-title">
            <div className={StyleComponent.card.title.text}>{title}</div>
        </div>
    );
}
