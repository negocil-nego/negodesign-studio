import type { FormIt } from "../../../molecules/form/data/types";

export type CheckboxProps = FormIt & {
  checked?: boolean;
  disabled?: boolean;
  label?: string;
  required?: boolean;
  onCheckedChange?: (checked: boolean) => void;
};