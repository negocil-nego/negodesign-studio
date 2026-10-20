// @ts-nocheck
import type { InputProps } from "../data/types";

export default function Input({ type, required, disabled, placeholder, value }: InputProps) {
    return (
        <input
            type={type}
            required={required}
            disabled={disabled}
            placeholder={placeholder}
            value={value}
            data-name="label"
        />
    );
}
