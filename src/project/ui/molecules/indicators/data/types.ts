export type IndicatorVariant =
  | "primary"
  | "secondary"
  | "success"
  | "danger"
  | "warning"
  | "info"
  | "light"
  | "dark";

export interface AlertItem {
  id: string;
  variant: IndicatorVariant;
  heading?: string;
  message: string;
  linkText?: string;
  dismissible?: boolean;
}

export interface BadgeItem {
  label: string;
  variant: IndicatorVariant;
  pill?: boolean;
}

export interface IndicatorsProps {
  alerts?: AlertItem[];
  badges?: BadgeItem[];
  onDismissAlert?: (id: string) => void;
}
