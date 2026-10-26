import type { ComponentPropsWithoutRef } from "react";
import type { InputProps } from "../data/types";
import { getIconClass } from "../../../../../../hooks/icon.svelte";
import { StyleComponent } from "../../../../../data/style.component";

type Props = InputProps & Omit<ComponentPropsWithoutRef<"input">, keyof InputProps>;

export default function Input({ id, icon, type, required, disabled, placeholder, value, className, ...rest }: Props) {
    const fieldClass =
        (StyleComponent.input as unknown as Record<string, { root?: string }>)[type ?? ""]?.root ??
        StyleComponent.input.form;

    return (
        <div className={fieldClass}>
            {icon && <i className={getIconClass(icon)} aria-hidden="true" />}
            <input
                {...rest}
                id={id}
                type={type}
                required={required}
                disabled={disabled}
                placeholder={placeholder}
                value={value}
                data-name="label"
                className={className}
            />
        </div>
    );
}