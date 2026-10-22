// @ts-nocheck
import type { SelectProps } from "../data/types";

export default function Select({ id, name, options, value, placeholder, disabled, required }: SelectProps) {
    return (
        <div className="input-form">
            <select id={id} name={name} defaultValue={value ?? ""} disabled={disabled} required={required} className="input-select">
                {placeholder && (
                    <option value="" disabled hidden>{placeholder}</option>
                )}
                {options.map((option) => (
                    <option key={option.value} value={option.value} disabled={option.disabled}>
                        {option.label}
                    </option>
                ))}
            </select>
        </div>
    );
}
