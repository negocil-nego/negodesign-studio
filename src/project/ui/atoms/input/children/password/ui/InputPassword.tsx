// @ts-nocheck
import Input from "../../../core/ui/Input";
import InputLabel from "../../label/ui/InputLabel";
import type { InputPasswordProps } from "../data/types";

export default function InputPassword({ labelProps, inputProps }: InputPasswordProps) {
  const componentProps = { ...inputProps, type: "password" as const };

  return labelProps
    ? <InputLabel labelProps={labelProps} inputProps={componentProps} />
    : <Input {...componentProps} />;
}
