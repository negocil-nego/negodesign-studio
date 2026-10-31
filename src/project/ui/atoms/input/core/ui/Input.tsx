import type { ComponentPropsWithoutRef } from "react";
import type { InputProps } from "../data/types";
import Icon from "../../../icon/ui/Icon";
import { StyleComponent } from "../../../../../data/style.component";

type Props = InputProps & Omit<ComponentPropsWithoutRef<"input">, keyof InputProps>;

export default function Input({ id, icon, type, required, disabled, placeholder, value, isInvalid = false, className, ...rest }: Props) {
    const fieldClass =
        (StyleComponent.input as unknown as Record<string, { root?: string }>)[type ?? ""]?.root ??
        StyleComponent.input.form;

    return (
        <div className={`${fieldClass} ${isInvalid ? StyleComponent.input.error : ""}`}>
            {icon && <Icon name={icon} className="shrink-0 text-zinc-400" />}
            <input
                {...rest}
                id={id}
                type={type}
                required={required}
                disabled={disabled}
                placeholder={placeholder}
                value={value}
                aria-invalid={isInvalid || undefined}
                data-name="label"
                className={className}
            />
        </div>
    );
}
