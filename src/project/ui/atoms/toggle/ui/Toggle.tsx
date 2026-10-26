// @ts-nocheck
import { useState } from "react";
import type { ToggleProps } from "../data/types";
import { StyleComponent } from "../../../../data/style.component";

export default function Toggle({ id, name, checked: initialChecked = false, disabled, label, onchange }: ToggleProps) {
    const [checked, setChecked] = useState(initialChecked);

    function handleChange() {
        const next = !checked;
        setChecked(next);
        onchange?.(next);
    }

    return (
        <label className={StyleComponent.toggle.root} htmlFor={id}>
            <button
                type="button"
                role="switch"
                id={id}
                name={name}
                aria-checked={checked}
                disabled={disabled}
                onClick={handleChange}
                className={`${StyleComponent.toggle.track} ${checked ? StyleComponent.toggle.on : ""}`}
            >
                <span className={StyleComponent.toggle.thumb} />
            </button>
            {label && <span className={StyleComponent.toggle.label}>{label}</span>}
        </label>
    );
}
