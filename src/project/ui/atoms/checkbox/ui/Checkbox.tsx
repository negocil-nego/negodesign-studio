import { useState } from "react";
import type { ComponentPropsWithoutRef, ChangeEvent } from "react";
import type { CheckboxProps } from "../data/types";
import { StyleComponent } from "../../../../data/style.component";

type Props = CheckboxProps & Omit<ComponentPropsWithoutRef<"input">, keyof CheckboxProps | "type">;

export default function Checkbox({ id, name, value, checked: initialChecked = false, disabled, label, required, onCheckedChange, onChange, className, ...rest }: Props) {
    const [checked, setChecked] = useState(initialChecked);

    function handleChange(event: ChangeEvent<HTMLInputElement>) {
        const next = event.target.checked;
        setChecked(next);
        onChange?.(event);
        onCheckedChange?.(next);
    }

    return (
        <label className={`${StyleComponent.checkbox.root} ${disabled ? StyleComponent.checkbox.disabled : ""}`} htmlFor={id}>
            <input
                {...rest}
                type="checkbox"
                id={id}
                name={name}
                value={value}
                checked={checked}
                disabled={disabled}
                required={required}
                onChange={handleChange}
                className={`${StyleComponent.checkbox.input} ${className ?? ""}`}
            />
            <span className={StyleComponent.checkbox.box}>
                {checked && <svg viewBox="0 0 10 8" fill="none" aria-hidden="true" className={StyleComponent.checkbox.tick}>
                    <path d="M1 4l3 3 5-6" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
                </svg>}
            </span>
            {label && <span className={StyleComponent.checkbox.label}>{label}</span>}
        </label>
    );
}