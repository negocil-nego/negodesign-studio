import type { FlagNamed } from "../../../../../hooks/flag.svelte";
import type { CountryPhoneOption } from "../../popover/data/country-phone";

export type FormComponent =
  | "select"
  | "checkbox"
  | "radio"
  | "toggle"
  | "textarea"
  | "input"
  | "input-label"
  | "input-age"
  | "input-date"
  | "input-email"
  | "input-number"
  | "input-password"
  | "input-phone"
  | "input-phone-flag"
  | "input-text"
  | "input-username"
  | "input-username-or-email"
  | "input-username-or-phone"
  | "input-otp"
  | "input-time";

export type FormIt = {
  /** Component rendered for this field. Input variants use inputProps.type for the HTML input type. */
  component?: FormComponent;
  /** Legacy selector for the original form controls. Prefer `component`. */
  type?: "select" | "checkbox" | "radio" | "toggle" | "textarea" | "text" | "email" | "password" | "number" | "tel" | "url" | "search" | "date" | "datetime-local" | "month" | "week" | "time";
  id?: string;
  name?: string;
  value?: string;
  length?: number;
  separatorEvery?: number;
  position?: "start" | "center";
  checked?: boolean;
  disabled?: boolean;
  required?: boolean;
  placeholder?: string;
  rows?: number;
  cols?: number;
  readonly?: boolean;
  maxlength?: number;
  label?: string;
  labelProps?: {
    label: string;
    for: string;
    id?: string;
    name?: string;
    value?: string;
  };
  inputProps?: {
    id?: string;
    name?: string;
    value?: string;
    icon?: string;
    type: "text" | "email" | "password" | "number" | "tel" | "url" | "search" | "date" | "datetime-local" | "month" | "week" | "time";
    required?: boolean;
    disabled?: boolean;
    placeholder?: string;
  };
  options?: Array<{
    value: string;
    label: string;
    icon?: string;
    disabled?: boolean;
  }>;
  flag?: FlagNamed[];
  locale?: string;
  onCountrySelect?: (country: CountryPhoneOption) => void;
  onCheckedChange?: (checked: boolean) => void;
  onValidityChange?: (isValid: boolean) => void;
  validationMode?: "any" | "username" | "email" | "phone" | "username-or-email" | "username-or-phone";
  errorMessage?: string;
  onValueChange?: (value: string) => void;
  onOptionSelect?: (option: { value: string; label: string; icon?: string; disabled?: boolean }) => void;
  /** Allow native HTML and ARIA props in field configuration. */
  class?: string;
  className?: string;
  style?: string;
  title?: string;
  autoComplete?: string;
  autoFocus?: boolean;
  form?: string;
  inputMode?: string;
  min?: number | string;
  max?: number | string;
  minLength?: number;
  maxLength?: number;
  pattern?: string;
  step?: number | string;
  multiple?: boolean;
  size?: number;
  readOnly?: boolean;
  spellCheck?: boolean;
  tabIndex?: number;
  [attribute: `aria-${string}`]: unknown;
  [attribute: `data-${string}`]: unknown;
};

export type FormProps = {
  style?: 'inline' | 'grid'
  className?: string
  fields?: FormIt[];
  onSubmit?: () => void;
};
