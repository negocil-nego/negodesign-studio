export type ToggleProps = {
  id: string;
  name?: string;
  checked?: boolean;
  disabled?: boolean;
  label?: string;
  onchange?: (checked: boolean) => void;
};
