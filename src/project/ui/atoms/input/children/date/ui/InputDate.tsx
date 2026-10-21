import Input from "../../../core/ui/Input";
import InputLabel from "../../label/ui/InputLabel";
import type { InputDateProps } from "../data/types";

export default function InputDate({ labelProps, inputProps }: InputDateProps) {
  const componentProps = { ...inputProps, type: "date" as const };

  return labelProps
    ? <InputLabel labelProps={labelProps} inputProps={componentProps} />
    : <Input {...componentProps} />;
}
