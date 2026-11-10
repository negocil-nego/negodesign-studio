export type ProgressVariant = "primary" | "success" | "info" | "warning" | "danger";

export interface ProgressBarSegment {
  value: number;
  variant?: ProgressVariant;
  striped?: boolean;
  animated?: boolean;
  label?: string;
}

export interface ProgressProps {
  value?: number;
  variant?: ProgressVariant;
  striped?: boolean;
  animated?: boolean;
  bars?: ProgressBarSegment[];
}
