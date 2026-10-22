// @ts-nocheck
import type { TextareaProps } from "../data/types";

export default function Textarea({ id, name, value, placeholder, rows = 4, cols, disabled, required, readonly, maxlength }: TextareaProps) {
    return (
        <div className="input-form">
            <textarea
                id={id}
                name={name}
                defaultValue={value}
                placeholder={placeholder}
                rows={rows}
                cols={cols}
                disabled={disabled}
                required={required}
                readOnly={readonly}
                maxLength={maxlength}
                data-name="textarea"
            />
        </div>
    );
}
