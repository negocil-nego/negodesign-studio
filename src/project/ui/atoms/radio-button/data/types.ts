export interface RadioButtonOption {
  value: string;
  label: string;
  disabled?: boolean;
}

export interface RadioButtonProps {
  id?: string;
  name?: string;
  options: RadioButtonOption[];
  value?: string;
  multiple?: boolean;
  values?: string[];
  disabled?: boolean;
  onValueChange?: (value: string) => void;
  onValuesChange?: (values: string[]) => void;
}
