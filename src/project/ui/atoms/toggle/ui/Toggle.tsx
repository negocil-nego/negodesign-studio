import { useState } from "react";
import type { ComponentPropsWithoutRef, MouseEvent } from "react";
import type { ToggleProps } from "../data/types";
import { StyleComponent } from "../../../../data/style.component";

type Props = ToggleProps & Omit<ComponentPropsWithoutRef<"button">, keyof ToggleProps | "type">;

export default function Toggle({ id, name, value, checked: initialChecked = false, disabled, label, onCheckedChange, onClick, className, ...rest }: Props) {
    const [checked, setChecked] = useState(initialChecked);

    function handleClick(event: MouseEvent<HTMLButtonElement>) {
        if (disabled) return;
        const next = !checked;
        setChecked(next);
        onClick?.(event);
        onCheckedChange?.(next);
    }

    return (
        <label className={StyleComponent.toggle.root} htmlFor={id}>
            <button
                {...rest}
                type="button"
                id={id}
                name={name}
                value={value}
                role="switch"
                aria-checked={checked}
                disabled={disabled}
                onClick={handleClick}
                className={`${StyleComponent.toggle.track} ${checked ? StyleComponent.toggle.on : ""} ${className ?? ""}`}
            >
                <span className={`${StyleComponent.toggle.thumb} ${checked ? StyleComponent.toggle.thumbOn : ""}`} />
            </button>
            {label && <span className={StyleComponent.toggle.label}>{typeof label === "string" ? label : label.label}</span>}
        </label>
    );
}