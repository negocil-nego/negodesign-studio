import type { FormIt } from "../../../molecules/form/data/types";
import type { LabelProps } from "../../label/data/types";

export type TextareaProps = FormIt & {
  placeholder?: string;
  rows?: number;
  cols?: number;
  disabled?: boolean;
  required?: boolean;
  readonly?: boolean;
  maxlength?: number;
  labelProps?: LabelProps;
};
