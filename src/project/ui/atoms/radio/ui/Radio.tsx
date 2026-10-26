// @ts-nocheck
import { useState } from "react";
import type { RadioProps } from "../data/types";
import { StyleComponent } from "../../../../data/style.component";

export default function Radio({ name, options, value: initialValue = "", disabled, onchange }: RadioProps) {
    const [value, setValue] = useState(initialValue);

    function handleChange(optionValue: string) {
        setValue(optionValue);
        onchange?.(optionValue);
    }

    return (
        <fieldset className={StyleComponent.radio.group} disabled={disabled}>
            {options.map((option) => (
                <label
                    key={option.value}
                    className={`${StyleComponent.radio.item} ${option.disabled || disabled ? StyleComponent.radio.disabled : ""}`}
                >
                    <input
                        type="radio"
                        name={name}
                        value={option.value}
                        checked={value === option.value}
                        disabled={option.disabled || disabled}
                        onChange={() => handleChange(option.value)}
                        className={StyleComponent.radio.input}
                    />
                    <span className={StyleComponent.radio.dot} />
                    <span className={StyleComponent.radio.label}>{option.label}</span>
                </label>
            ))}
        </fieldset>
    );
}
