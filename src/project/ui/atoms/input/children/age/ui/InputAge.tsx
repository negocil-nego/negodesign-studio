// @ts-nocheck
import Input from "../../../core/ui/Input";
import InputLabel from "../../label/ui/InputLabel";
import type { InputAgeProps } from "../data/types";

export default function InputAge({ labelProps, inputProps }: InputAgeProps) {
  const componentProps = { ...inputProps, type: "number" as const };

  return labelProps
    ? <InputLabel labelProps={labelProps} inputProps={componentProps} />
    : <Input {...componentProps} />;
}
