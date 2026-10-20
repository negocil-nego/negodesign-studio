// @ts-nocheck
import Input from "../../../core/ui/Input";
import InputLabel from "../../label/ui/InputLabel";
import type { InputEmailProps } from "../data/types";

export default function InputEmail({ labelProps, inputProps }: InputEmailProps) {
    const emailProps = { ...inputProps, type: "email" };

    return (
        <>
            {labelProps ? (
                <InputLabel labelProps={labelProps} inputProps={emailProps} />
            ) : (
                <Input {...emailProps} />
            )}
        </>
    );
}
