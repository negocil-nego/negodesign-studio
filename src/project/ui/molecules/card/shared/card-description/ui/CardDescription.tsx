// @ts-nocheck
import type { CardDescriptionProps } from "../data/types";

export default function CardDescription({ description, className }: CardDescriptionProps) {
    return (
        <div className={`card-description ${className ?? ""}`} id="card-description">
            <div className="text-justify line-clamp-2 xl:line-clamp-3 2xl:line-clamp-4">
                {description}
            </div>
        </div>
    );
}
