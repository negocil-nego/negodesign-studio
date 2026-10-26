import type { IconNamed } from "../../../../../../hooks/icon.svelte";
import type { FormIt } from "../../../../molecules/form/data/types";

export type InputType =
  | "text" | "email" | "password" | "number" | "tel" | "url" | "search"
  | "date" | "datetime-local" | "month" | "week" | "time";

/** Component-specific input props. Framework-native attributes are added by each adapter. */
export type InputProps = {
  id?: string;
  name?: string;
  value?: string | number | readonly string[];
  icon?: IconNamed | string;
  type: InputType;
  required?: boolean;
  disabled?: boolean;
  placeholder?: string;
};

export type InputWithoutLabelProps = FormIt & {
  inputProps: InputProps;
};