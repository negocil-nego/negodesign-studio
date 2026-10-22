// @ts-nocheck
import { useState } from "react";
import type { RadioProps } from "../data/types";

export default function Radio({ name, options, value: initialValue = "", disabled, onchange }: RadioProps) {
    const [value, setValue] = useState(initialValue);

    function handleChange(optionValue: string) {
        setValue(optionValue);
        onchange?.(optionValue);
    }

    return (
        <fieldset className="radio-group" disabled={disabled}>
            {options.map((option) => (
                <label key={option.value} className={`radio-item${option.disabled || disabled ? " radio-disabled" : ""}`}>
                    <input
                        type="radio"
                        name={name}
                        value={option.value}
                        checked={value === option.value}
                        disabled={option.disabled || disabled}
                        onChange={() => handleChange(option.value)}
                        className="radio-input"
                    />
                    <span className="radio-dot" />
                    <span className="radio-label">{option.label}</span>
                </label>
            ))}
        </fieldset>
    );
}
