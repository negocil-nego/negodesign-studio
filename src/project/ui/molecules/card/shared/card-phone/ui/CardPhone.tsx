// @ts-nocheck
import type { CardPhoneProps } from "../data/types";

export default function CardPhone({ phone }: CardPhoneProps) {
    return (
        <div className="card-phone" id="card-phone">
            <div className="card-phone-number">{phone}</div>
        </div>
    );
}
