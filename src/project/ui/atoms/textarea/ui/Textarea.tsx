// @ts-nocheck
import type { TextareaProps } from "../data/types";
import { StyleComponent } from "../../../../data/style.component";
import Label from "../../label/ui/label";

export default function Textarea({ id, name, value, placeholder, rows = 4, cols, disabled, required, readonly, maxlength,  labelProps }: TextareaProps) {
    return (
        <div className={StyleComponent.textarea.form}>
            {labelProps && (
                <Label {...labelProps} />
            )}
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
                className={StyleComponent.textarea.default}
            />
        </div>
    );
}
