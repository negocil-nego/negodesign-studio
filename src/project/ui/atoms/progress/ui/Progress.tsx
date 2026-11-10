// @ts-nocheck
import { StyleComponent } from "../../../../data/style.component";
import type { ProgressBarSegment, ProgressProps } from "../data/types";

export default function Progress({
  value = 25,
  variant = "primary",
  striped = false,
  animated = false,
  bars = [],
}: ProgressProps) {
  const resolvedBars: ProgressBarSegment[] = bars.length
    ? bars
    : [{ value, variant, striped, animated }];

  return (
    <div className={StyleComponent.progress.track} role="progressbar" aria-valuemin={0} aria-valuemax={100}>
      {resolvedBars.map((bar, idx) => (
        <div
          key={idx}
          className={`${StyleComponent.progress.bar} bg-${bar.variant ?? "primary"} ${
            bar.striped || bar.animated ? StyleComponent.progress.striped : ""
          } ${bar.animated ? StyleComponent.progress.animated : ""}`}
          style={{ width: `${Math.max(0, Math.min(100, bar.value))}%` }}
        >
          {bar.label}
        </div>
      ))}
    </div>
  );
}
