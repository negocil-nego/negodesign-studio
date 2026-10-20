// @ts-nocheck
import Input from "../../../core/ui/Input";
import InputLabel from "../../label/ui/InputLabel";
import type { InputTextProps } from "../data/types";

export default function InputText({ labelProps, inputProps }: InputTextProps) {
    const phoneProps = { ...inputProps, type: "tel" };

    return (
        <>
            {labelProps ? (
                <InputLabel labelProps={labelProps} inputProps={phoneProps} />
            ) : (
                <Input {...phoneProps} />
            )}
        </>
    );
}