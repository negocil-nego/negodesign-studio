// @ts-nocheck
import { useState } from "react";
import { StyleComponent } from "../../../../data/style.component";
import type { AlertItem, BadgeItem, IndicatorsProps } from "../data/types";

const defaultAlerts: AlertItem[] = [
  {
    id: "warn-1",
    variant: "warning",
    heading: "Warning!",
    message: "Best check yo self, you're not looking too good.",
    dismissible: true,
  },
  {
    id: "danger-1",
    variant: "danger",
    message: "Oh snap! Change a few things up and try submitting again.",
    dismissible: true,
  },
  {
    id: "success-1",
    variant: "success",
    message: "Well done! You successfully read this important alert message.",
    dismissible: true,
  },
];

const defaultBadges: BadgeItem[] = [
  { label: "Primary", variant: "primary" },
  { label: "Secondary", variant: "secondary" },
  { label: "Success", variant: "success" },
  { label: "Danger", variant: "danger" },
  { label: "Warning", variant: "warning" },
  { label: "Info", variant: "info" },
];

export default function Indicators({
  alerts = defaultAlerts,
  badges = defaultBadges,
  onDismissAlert,
}: IndicatorsProps) {
  const [dismissed, setDismissed] = useState<string[]>([]);
  const visible = alerts.filter((a) => !dismissed.includes(a.id));

  function dismiss(id: string) {
    setDismissed((prev) => [...prev, id]);
    onDismissAlert?.(id);
  }

  return (
    <div className={StyleComponent.indicators.root}>
      <div className="space-y-3">
        {visible.map((alert) => (
          <div
            key={alert.id}
            className={`${StyleComponent.indicators.alert} alert-${alert.variant}`}
            role="alert"
          >
            {alert.heading && (
              <h4 className={StyleComponent.indicators.alertHeading}>{alert.heading}</h4>
            )}
            <p>{alert.message}</p>
            {alert.dismissible !== false && (
              <button
                type="button"
                className={StyleComponent.indicators.alertClose}
                onClick={() => dismiss(alert.id)}
              >
                &times;
              </button>
            )}
          </div>
        ))}
      </div>
      <div className={StyleComponent.indicators.badgesRow}>
        {badges.map((b, idx) => (
          <span
            key={idx}
            className={`${StyleComponent.indicators.badge} text-bg-${b.variant} ${
              b.pill ? StyleComponent.indicators.badgePill : ""
            }`}
          >
            {b.label}
          </span>
        ))}
      </div>
    </div>
  );
}
