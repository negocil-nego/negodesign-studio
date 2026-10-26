import { useState } from "react";
import type { ComponentPropsWithoutRef, ChangeEvent } from "react";
import type { RadioProps } from "../data/types";
import { StyleComponent } from "../../../../data/style.component";

type Props = RadioProps & Omit<ComponentPropsWithoutRef<"input">, keyof RadioProps | "type">;

export default function Radio({ id, name, options, value: initialValue = "", disabled, onValueChange, onChange, className, ...rest }: Props) {
    const [value, setValue] = useState(initialValue);

    function handleChange(optionValue: string, event: ChangeEvent<HTMLInputElement>) {
        setValue(optionValue);
        onChange?.(event);
        onValueChange?.(optionValue);
    }

    return (
        <fieldset className={StyleComponent.radio.group} disabled={disabled}>
            {options.map((option, index) => {
                const optionId = id ? `${id}-${index}` : undefined;
                return <label key={option.value} htmlFor={optionId} className={`${StyleComponent.radio.item} ${option.disabled || disabled ? StyleComponent.radio.disabled : ""}`}>
                    <input
                        {...rest}
                        type="radio"
                        id={optionId}
                        name={name}
                        value={option.value}
                        checked={value === option.value}
                        disabled={option.disabled || disabled}
                        onChange={(event) => handleChange(option.value, event)}
                        className={`${StyleComponent.radio.input} ${className ?? ""}`}
                    />
                    <span className={StyleComponent.radio.dot} />
                    <span className={StyleComponent.radio.label}>{option.label}</span>
                </label>;
            })}
        </fieldset>
    );
}