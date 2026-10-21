// @ts-nocheck
import type { InputProps } from "../data/types";
import { getIconClass } from "../../../../../../hooks/icon.svelte";

function InputField({ type, required, disabled, placeholder, value }) {
    return (
        <input
            type={type}
            required={required}
            disabled={disabled}
            placeholder={placeholder}
            value={value}
            data-name="label"
        />
    );
}

export default function Input({ icon, type, required, disabled, placeholder, value }: InputProps) {
    const fieldProps = { type, required, disabled, placeholder, value };

    return (
        <div className="input-form">
            {icon && <i className={getIconClass(icon)} aria-hidden="true" />}
            <InputField {...fieldProps} />
        </div>
    );
}
