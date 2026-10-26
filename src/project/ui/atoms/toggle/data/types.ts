import type { FormIt } from "../../../molecules/form/data/types";
import type { LabelProps } from "../../label/data/types";

export type ToggleProps = Omit<FormIt, "label"> & {
  checked?: boolean;
  disabled?: boolean;
  label?: LabelProps | string;
  labelFirst?: boolean;
  onCheckedChange?: (checked: boolean) => void;
};