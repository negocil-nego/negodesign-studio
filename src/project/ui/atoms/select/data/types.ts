export type SelectOption = {
  value: string;
  label: string;
  icon?: string;
  disabled?: boolean;
};

export type SelectProps = {
  id: string;
  name?: string;
  label?: string;
  options: SelectOption[];
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  required?: boolean;
  onselect?: (option: SelectOption) => void;
};