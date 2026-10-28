export type InputOtpLabelProps = { label: string; for: string };

export type InputOtpProps = {
  id?: string;
  name?: string;
  value?: string;
  length?: number;
  separatorEvery?: number;
  labelProps?: InputOtpLabelProps;
  label?: string;
  disabled?: boolean;
  required?: boolean;
  onValueChange?: (value: string) => void;
};
