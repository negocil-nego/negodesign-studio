// @ts-nocheck
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardRatingProps } from "../data/types";
import Icon from "../../../../../atoms/icon/ui/Icon";

export default function CardRating({ rating }: CardRatingProps) {
    const MAX_STARS = 5;
    const filled = Math.min(MAX_STARS, Math.max(0, Math.round(rating)));

    return (
        <div className={StyleComponent.card.rating.root} role="img" aria-label={`rating ${filled} de ${MAX_STARS}`}>
            {Array.from({ length: MAX_STARS }).map((_, index) => (
                <Icon
                    key={index}
                    name="Star"
                    className={index < filled ? StyleComponent.card.rating.filled : ""}
                />
            ))}
        </div>
    );
}
