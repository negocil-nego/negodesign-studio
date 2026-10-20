// @ts-nocheck
import Input from "../../../core/ui/Input";
import InputLabel from "../../label/ui/InputLabel";
import type { InputPhoneProps } from "../data/types";

export default function InputPhone({ labelProps, inputProps }: InputPhoneProps) {
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
