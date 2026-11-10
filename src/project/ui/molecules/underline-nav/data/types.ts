export interface UnderlineNavItem {
  value: string;
  label: string;
  disabled?: boolean;
}

export interface UnderlineNavProps {
  items: UnderlineNavItem[];
  value?: string;
  onValueChange?: (value: string) => void;
}
