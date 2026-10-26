import type { ComponentPropsWithoutRef } from "react";
import type { TextareaProps } from "../data/types";
import { StyleComponent } from "../../../../data/style.component";
import Label from "../../label/ui/label";

type Props = TextareaProps & Omit<ComponentPropsWithoutRef<"textarea">, keyof TextareaProps | "defaultValue">;

export default function Textarea({ id, name, value, placeholder, rows = 4, cols, disabled, required, readonly, maxlength, labelProps, className, readOnly: nativeReadOnly, maxLength: nativeMaxLength, ...rest }: Props) {
    return (
        <div className={StyleComponent.textarea.form}>
            {labelProps && <Label {...labelProps} />}
            <textarea
                {...rest}
                id={id}
                name={name}
                defaultValue={value}
                placeholder={placeholder}
                rows={rows}
                cols={cols}
                disabled={disabled}
                required={required}
                readOnly={readonly ?? nativeReadOnly}
                maxLength={maxlength ?? nativeMaxLength}
                data-name="textarea"
                className={`${StyleComponent.textarea.default} ${className ?? ""}`}
            />
        </div>
    );
}