export type CheckboxProps = {
  id: string;
  name?: string;
  checked?: boolean;
  disabled?: boolean;
  label?: string;
  required?: boolean;
  onchange?: (checked: boolean) => void;
};
