// @ts-nocheck
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardMinFileProps } from "../data/types";
import CardImage from "../../card-image/ui/CardImage";

export default function CardMinFile({ imageProps, active = false, onClick }: CardMinFileProps) {
    return (
        <button
            type="button"
            className={`${StyleComponent.card.cardMinFile.root} ${active ? StyleComponent.card.cardMinFile.active : ""}`}
            onClick={onClick}
        >
            {imageProps && <CardImage {...imageProps} />}
        </button>
    );
}
