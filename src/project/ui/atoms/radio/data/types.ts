export type RadioOption = {
  value: string;
  label: string;
  disabled?: boolean;
};

export type RadioProps = {
  name: string;
  options: RadioOption[];
  value?: string;
  disabled?: boolean;
  onchange?: (value: string) => void;
};
