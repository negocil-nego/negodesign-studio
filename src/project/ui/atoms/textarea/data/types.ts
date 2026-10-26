import type { LabelProps } from "../../label/data/types";

export type TextareaProps = {
  id: string;
  name?: string;
  value?: string;
  placeholder?: string;
  rows?: number;
  cols?: number;
  disabled?: boolean;
  required?: boolean;
  readonly?: boolean;
  maxlength?: number;
  labelProps?: LabelProps;
};
