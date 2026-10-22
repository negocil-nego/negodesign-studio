// @ts-nocheck
import { useState } from "react";
import type { CheckboxProps } from "../data/types";

export default function Checkbox({ id, name, checked: initialChecked = false, disabled, label, required, onchange }: CheckboxProps) {
    const [checked, setChecked] = useState(initialChecked);

    function handleChange(e: React.ChangeEvent<HTMLInputElement>) {
        const next = e.target.checked;
        setChecked(next);
        onchange?.(next);
    }

    return (
        <label className={`checkbox-root${disabled ? " checkbox-disabled" : ""}`} htmlFor={id}>
            <input
                type="checkbox"
                id={id}
                name={name}
                checked={checked}
                disabled={disabled}
                required={required}
                onChange={handleChange}
                className="checkbox-input"
            />
            <span className="checkbox-box">
                {checked && (
                    <svg viewBox="0 0 10 8" fill="none" aria-hidden="true" className="checkbox-tick">
                        <path d="M1 4l3 3 5-6" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
                    </svg>
                )}
            </span>
            {label && <span className="checkbox-label">{label}</span>}
        </label>
    );
}
