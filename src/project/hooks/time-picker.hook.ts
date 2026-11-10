export const TIME_HOURS = Array.from({ length: 12 }, (_, i) =>
  (i + 1).toString().padStart(2, "0"),
);

export const TIME_MINUTES = Array.from({ length: 12 }, (_, i) =>
  (i * 5).toString().padStart(2, "0"),
);

export const TIME_PERIODS = ["AM", "PM"] as const;

export type ParsedTime = {
  hour: string;
  minute: string;
  period: string;
};

export function parseTimeValue(value?: string): ParsedTime {
  if (value) {
    const match = value.match(/(\d{2}):(\d{2})\s?(AM|PM)/i);
    if (match) {
      return {
        hour: match[1],
        minute: match[2],
        period: match[3].toUpperCase(),
      };
    }
  }
  return { hour: "02", minute: "30", period: "AM" };
}

export function formatTimeDisplay(hour: string, minute: string, period: string): string {
  return hour && minute ? `${hour}:${minute} ${period}` : "-- : --";
}
