// @ts-nocheck
import type { CardProps } from "../data/types";

export default function Card({ ...restProps }: CardProps) {
    return (
        <div className="card" id="card">
            <div {...restProps} />
        </div>
    );
}
