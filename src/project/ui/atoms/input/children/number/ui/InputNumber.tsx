import Input from "../../../core/ui/Input";
import InputLabel from "../../label/ui/InputLabel";
import type { InputNumberProps } from "../data/types";

export default function InputNumber({ labelProps, inputProps }: InputNumberProps) {
  const componentProps = { ...inputProps, type: "number" as const };

  return labelProps
    ? <InputLabel labelProps={labelProps} inputProps={componentProps} />
    : <Input {...componentProps} />;
}
