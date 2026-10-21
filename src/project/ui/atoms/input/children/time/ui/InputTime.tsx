import Input from "../../../core/ui/Input";
import InputLabel from "../../label/ui/InputLabel";
import type { InputTimeProps } from "../data/types";

export default function InputTime({ labelProps, inputProps }: InputTimeProps) {
  const componentProps = { ...inputProps, type: "time" as const };

  return labelProps
    ? <InputLabel labelProps={labelProps} inputProps={componentProps} />
    : <Input {...componentProps} />;
}
