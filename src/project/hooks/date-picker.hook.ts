export type CalendarMonthView = {
  monthName: string;
  weekdays: string[];
  days: number[];
  paddingDays: number[];
};

export function formatLongDate(locale: string, year: number, month: number, day: number): string {
  return new Intl.DateTimeFormat(locale, { dateStyle: "long" }).format(
    new Date(year, month, day),
  );
}

export function computeCalendarMonth(locale: string, year: number, month: number): CalendarMonthView {
  const monthName = new Intl.DateTimeFormat(locale, { month: "long" }).format(
    new Date(year, month, 1),
  );
  const weekdays = Array.from({ length: 7 }, (_, index) =>
    new Intl.DateTimeFormat(locale, { weekday: "short" }).format(
      new Date(2023, 0, index + 2),
    ),
  );
  const daysInMonth = new Date(year, month + 1, 0).getDate();
  const startDay = (new Date(year, month, 1).getDay() + 6) % 7;
  const days = Array.from({ length: daysInMonth }, (_, i) => i + 1);
  const prevDaysCount = new Date(year, month, 0).getDate();
  const paddingDays = Array.from(
    { length: startDay },
    (_, i) => prevDaysCount - startDay + i + 1,
  );
  return { monthName, weekdays, days, paddingDays };
}

export function stepCalendarMonth(year: number, month: number, delta: number): { year: number; month: number } {
  const next = month + delta;
  if (next > 11) return { year: year + 1, month: 0 };
  if (next < 0) return { year: year - 1, month: 11 };
  return { year, month: next };
}
