import type { FormIt } from "../../../molecules/form/data/types";

export type SelectOption = {
  value: string;
  label: string;
  icon?: string;
  disabled?: boolean;
};

export type SelectProps = FormIt & {
  label?: string;
  options: SelectOption[];
  placeholder?: string;
  disabled?: boolean;
  required?: boolean;
  onOptionSelect?: (option: SelectOption) => void;
};