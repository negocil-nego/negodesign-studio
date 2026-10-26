import type { FormIt } from "../../../molecules/form/data/types";

export type RadioOption = {
  value: string;
  label: string;
  disabled?: boolean;
};

export type RadioProps = FormIt & {
  options: RadioOption[];
  disabled?: boolean;
  onValueChange?: (value: string) => void;
};