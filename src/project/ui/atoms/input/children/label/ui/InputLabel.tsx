// @ts-nocheck
import type { InputLabelProps } from "../data/types";
import Input from "../../../core/ui/Input";
import Label from "../../../../label/ui/Label";

export default function InputLabel({ labelProps, inputProps }: InputLabelProps) {
    return (
        <>
            <Label {...labelProps} />
            <Input {...inputProps} />
        </>
    );
}