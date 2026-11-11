// @ts-nocheck
import type { LabelProps } from "../data/types";

export default function Label({ label, for: htmlFor }: LabelProps) {
    return (
        <label htmlFor={htmlFor} data-name="label">
            {label}
        </label>
    );
}