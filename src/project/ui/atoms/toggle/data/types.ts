import type { LabelProps } from "../../label/data/types";

export type ToggleProps = {
  id: string;
  name?: string;
  checked?: boolean;
  disabled?: boolean;
  label?: LabelProps;
  labelFirst?: boolean;
  onchange?: (checked: boolean) => void;
};
