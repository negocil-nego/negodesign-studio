// @ts-nocheck
import type { CardTitleProps } from "../data/types";

export default function CardTitle({ title, className }: CardTitleProps) {
    return (
        <div className={`card-title ${className ?? ""}`} id="card-title">
            <div className="card-title-text">{title}</div>
        </div>
    );
}
