export interface InputRangeProps {
  id?: string;
  name?: string;
  label?: string;
  value?: number;
  min?: number;
  max?: number;
  step?: number;
  disabled?: boolean;
  showValue?: boolean;
  hint?: string;
  onValueChange?: (value: number) => void;
}
