// @ts-nocheck
import type { InputProps } from "../data/types";

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

    if (icon) {
        return (
            <div className="flex items-center">
                <img src={`/icons/${icon}`} alt="icon" />
                <InputField {...fieldProps} />
            </div>
        );
    }

    return <InputField {...fieldProps} />;
}
