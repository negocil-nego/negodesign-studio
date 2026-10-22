// @ts-nocheck
import { useId, useState } from "react";
import Icon from "../../../../icon/ui/Icon";
import type { InputDateProps } from "../data/types";

export default function InputDate({ labelProps, inputProps, value = "November 21, 2023", onselect }: InputDateProps & { value?: string; onselect?: (date: string) => void }) {
  const popoverId = useId();
  const [currentYear, setCurrentYear] = useState(2023);
  const [currentMonthIndex, setCurrentMonthIndex] = useState(10);
  const [selectedDay, setSelectedDay] = useState(21);
  const [dateStr, setDateStr] = useState(inputProps?.value ?? value);

  const monthNames = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December"
  ];

  const currentMonthName = monthNames[currentMonthIndex];
  const daysInMonth = new Date(currentYear, currentMonthIndex + 1, 0).getDate();
  const firstDayOfWeek = (new Date(currentYear, currentMonthIndex, 1).getDay() + 6) % 7;
  const days = Array.from({ length: daysInMonth }, (_, i) => i + 1);
  const prevMonthDays = new Date(currentYear, currentMonthIndex, 0).getDate();
  const paddingDaysBefore = Array.from({ length: firstDayOfWeek }, (_, i) => prevMonthDays - firstDayOfWeek + i + 1);

  function nextMonth() {
    if (currentMonthIndex === 11) {
      setCurrentMonthIndex(0);
      setCurrentYear(currentYear + 1);
    } else {
      setCurrentMonthIndex(currentMonthIndex + 1);
    }
  }

  function prevMonth() {
    if (currentMonthIndex === 0) {
      setCurrentMonthIndex(11);
      setCurrentYear(currentYear - 1);
    } else {
      setCurrentMonthIndex(currentMonthIndex - 1);
    }
  }

  function selectDate(day: number) {
    setSelectedDay(day);
    const nextVal = `${currentMonthName} ${day}, ${currentYear}`;
    setDateStr(nextVal);
    onselect?.(nextVal);
  }

  return (
    <div className="relative w-full">
      <div className="popover w-full">
        <button
          type="button"
          id={inputProps?.id}
          disabled={inputProps?.disabled}
          popoverTarget={popoverId}
          aria-haspopup="dialog"
          className="input-form w-full cursor-pointer flex items-center justify-between text-left"
        >
          <div className="flex flex-col w-full min-w-0">
            {labelProps && (
              <span className="text-[10px] uppercase font-semibold text-zinc-500 select-none">
                {labelProps.label}
              </span>
            )}
            <div className="flex items-center gap-2 mt-0.5">
              <span className="text-sm">
                <Icon name="Label" />
              </span>
              <span className="text-sm font-medium truncate">
                {dateStr}
              </span>
            </div>
          </div>
          <div className="flex flex-col text-zinc-400 text-xs ml-2 select-none shrink-0" aria-hidden="true">
            <span><Icon name="HelpSquare" /></span>
            <span><Icon name="HelpSquare" /></span>
          </div>
        </button>

        <div
          id={popoverId}
          popover="auto"
          role="dialog"
          aria-label={labelProps?.label ?? "Pick a date"}
          className="mt-1.5 text-zinc-200 z-50 outline-none"
        >
          <div className="flex items-center justify-between mb-4">
            <button type="button" onClick={prevMonth} className="p-1.5 rounded hover:bg-blender-hover cursor-pointer">
              <Icon name="HelpSquare" />
            </button>
            <span className="font-bold text-sm text-zinc-200">{currentMonthName} {currentYear}</span>
            <button type="button" onClick={nextMonth} className="p-1.5 rounded hover:bg-blender-hover cursor-pointer">
              <Icon name="HelpSquare" />
            </button>
          </div>

          <div className="grid grid-cols-7 text-center text-xs font-medium text-zinc-500 mb-3">
            <span>Mon</span><span>Tue</span><span>Wed</span><span>Thu</span><span>Fri</span><span>Sat</span><span>Sun</span>
          </div>

          <div className="grid grid-cols-7 gap-y-1.5 text-center text-sm font-medium">
            {paddingDaysBefore.map((padDay, idx) => (
              <span key={`pad-${idx}`} className="text-zinc-600 py-1 opacity-40">{padDay}</span>
            ))}
            {days.map((day) => (
              <button
                key={day}
                type="button"
                popoverTarget={popoverId}
                popoverTargetAction="hide"
                onClick={() => selectDate(day)}
                className={`w-8 h-8 mx-auto flex items-center justify-center rounded-full transition-all cursor-pointer ${day === selectedDay ? "bg-orange-500 text-white font-bold shadow-md" : "text-zinc-300 hover:bg-blender-hover"
                  }`}
              >
                {day}
              </button>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}