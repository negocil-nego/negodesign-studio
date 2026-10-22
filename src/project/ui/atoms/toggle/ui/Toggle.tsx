// @ts-nocheck
import { useState } from "react";
import type { ToggleProps } from "../data/types";

export default function Toggle({ id, name, checked: initialChecked = false, disabled, label, onchange }: ToggleProps) {
    const [checked, setChecked] = useState(initialChecked);

    function handleChange() {
        const next = !checked;
        setChecked(next);
        onchange?.(next);
    }

    return (
        <label className="toggle-root" htmlFor={id}>
            <button
                type="button"
                role="switch"
                id={id}
                name={name}
                aria-checked={checked}
                disabled={disabled}
                onClick={handleChange}
                className={`toggle-track${checked ? " toggle-on" : ""}`}
            >
                <span className="toggle-thumb" />
            </button>
            {label && <span className="toggle-label">{label}</span>}
        </label>
    );
}
