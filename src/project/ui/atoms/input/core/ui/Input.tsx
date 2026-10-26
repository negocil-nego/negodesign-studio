// @ts-nocheck
import type { InputProps } from "../data/types";
import { getIconClass } from "../../../../../../hooks/icon.svelte";
import { StyleComponent } from "../../../../../data/style.component";

export default function Input({ id, icon, type, required, disabled, placeholder, value }: InputProps) {
    const fieldClass =
        (StyleComponent.input as unknown as Record<string, { root?: string }>)[type ?? ""]?.root ??
        StyleComponent.input.form;

    return (
        <div className={fieldClass}>
            {icon && <i className={getIconClass(icon)} aria-hidden="true" />}
            <input
                id={id}
                type={type}
                required={required}
                disabled={disabled}
                placeholder={placeholder}
                value={value}
                data-name="label"
            />
        </div>
    );
}
