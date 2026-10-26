// @ts-nocheck
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardDescriptionProps } from "../data/types";

export default function CardDescription({ description, className }: CardDescriptionProps) {
    return (
        <div className={`card-description ${className ?? ""}`} id="card-description">
            <div className={StyleComponent.card.description}>
                {description}
            </div>
        </div>
    );
}
