import type { InputWithoutLabelProps } from "../../label/data/types";

export type UsernameValidationMode =
  | "any"
  | "username"
  | "email"
  | "phone"
  | "username-or-email"
  | "username-or-phone";

export type InputUsernameProps = InputWithoutLabelProps & {
  validationMode?: UsernameValidationMode;
  errorMessage?: string;
  onValidityChange?: (isValid: boolean) => void;
};
