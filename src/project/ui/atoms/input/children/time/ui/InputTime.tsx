// @ts-nocheck
import { useId, useState, useEffect } from "react";
import Icon from "../../../../icon/ui/Icon";
import type { InputTimeProps } from "../data/types";

export default function InputTime({
  labelProps,
  inputProps,
  value = "",
  onselect,
}: InputTimeProps & {
  value?: string;
  onselect?: (time: string) => void;
}) {
  const popoverId = useId();
  const [selectedHour, setSelectedHour] = useState("02");
  const [selectedMinute, setSelectedMinute] = useState("30");
  const [selectedPeriod, setSelectedPeriod] = useState("AM");
  const [currentTime, setCurrentTime] = useState(value);

  useEffect(() => {
    if (value) {
      const match = value.match(/(\d{2}):(\d{2})\s?(AM|PM)/i);
      if (match) {
        setSelectedHour(match[1]);
        setSelectedMinute(match[2]);
        setSelectedPeriod(match[3].toUpperCase());
        setCurrentTime(value);
      }
    }
  }, [value]);

  const displayTime =
    selectedHour && selectedMinute
      ? `${selectedHour}:${selectedMinute} ${selectedPeriod}`
      : "-- : --";

  function handleSelect(h: string, m: string, p: string) {
    const nextVal = `${h}:${m} ${p}`;
    setCurrentTime(nextVal);
    onselect?.(nextVal);
  }

  function clearTime(e: React.MouseEvent) {
    e.stopPropagation();
    setCurrentTime("");
    setSelectedHour("");
    setSelectedMinute("");
    setSelectedPeriod("AM");
    onselect?.("");
  }

  const hours = Array.from({ length: 12 }, (_, i) =>
    (i + 1).toString().padStart(2, "0"),
  );
  const minutes = Array.from({ length: 60 / 5 }, (_, i) =>
    (i * 5).toString().padStart(2, "0"),
  );
  const periods = ["AM", "PM"];

  return (
    <div className="relative w-full">
      <div className="popover w-full">
        <button
          type="button"
          id={inputProps?.id}
          disabled={inputProps?.disabled}
          popoverTarget={popoverId}
          aria-haspopup="dialog"
          className="flex w-full min-w-0 items-center justify-between rounded-md border border-blender-border bg-blender-surface px-3 py-2 text-sm text-zinc-200 focus-within:border-blue-500 focus-within:ring-1 focus-within:ring-blue-500 cursor-pointer text-left"
        >
          <div className="flex flex-col w-full min-w-0 items-center">
            {labelProps && (
              <span className="text-[10px] mt-2 uppercase font-semibold text-zinc-500 select-none">
                {labelProps.label}
              </span>
            )}
            <div className="flex items-center gap-2 mt-0.5">
              <span className="text-sm text-zinc-400">
                <Icon name="Time" />
              </span>
              <span className="text-sm font-medium truncate">
                {displayTime}
              </span>
            </div>
          </div>

          <div className="flex items-center gap-1">
            {currentTime && (
              <button
                type="button"
                className="p-1 rounded-full hover:bg-blender-hover text-zinc-500 hover:text-zinc-300 transition-colors"
                onClick={clearTime}
                aria-label="Clear time"
              >
                <Icon name="Cancel" />
              </button>
            )}
            <div
              className="flex flex-col text-zinc-400 text-[10px] ml-1 select-none shrink-0"
              aria-hidden="true"
            >
              <span>▲</span>
              <span>▼</span>
            </div>
          </div>
        </button>

        <div
          id={popoverId}
          popover="auto"
          role="dialog"
          aria-label={labelProps?.label ?? "Pick a time"}
          className="mt-1.5 p-3 rounded-lg border border-blender-border bg-blender-surface shadow-xl z-50 outline-none w-56"
        >
          <div className="flex justify-between gap-2 h-48">
            {/* Hours */}
            <div className="flex-1 flex flex-col overflow-y-auto no-scrollbar snap-y snap-mandatory">
              {hours.map((h) => (
                <button
                  key={h}
                  type="button"
                  className={`flex items-center justify-center py-2 text-sm font-medium transition-colors cursor-pointer snap-center shrink-0 ${
                    selectedHour === h
                      ? "bg-blue-600 text-white rounded-md"
                      : "text-zinc-500 hover:text-zinc-300"
                  }`}
                  onClick={() => {
                    setSelectedHour(h);
                    handleSelect(h, selectedMinute, selectedPeriod);
                  }}
                >
                  {h}
                </button>
              ))}
            </div>

            <div className="w-[1px] bg-blender-border/50 self-stretch my-2"></div>

            {/* Minutes */}
            <div className="flex-1 flex flex-col overflow-y-auto no-scrollbar snap-y snap-mandatory">
              {minutes.map((m) => (
                <button
                  key={m}
                  type="button"
                  className={`flex items-center justify-center py-2 text-sm font-medium transition-colors cursor-pointer snap-center shrink-0 ${
                    selectedMinute === m
                      ? "bg-blue-600 text-white rounded-md"
                      : "text-zinc-500 hover:text-zinc-300"
                  }`}
                  onClick={() => {
                    setSelectedMinute(m);
                    handleSelect(selectedHour, m, selectedPeriod);
                  }}
                >
                  {m}
                </button>
              ))}
            </div>

            <div className="w-[1px] bg-blender-border/50 self-stretch my-2"></div>

            {/* Period */}
            <div className="flex-1 flex flex-col overflow-y-auto no-scrollbar snap-y snap-mandatory">
              {periods.map((p) => (
                <button
                  key={p}
                  type="button"
                  className={`flex items-center justify-center py-2 text-sm font-medium transition-colors cursor-pointer snap-center shrink-0 ${
                    selectedPeriod === p
                      ? "bg-blue-600 text-white rounded-md"
                      : "text-zinc-500 hover:text-zinc-300"
                  }`}
                  onClick={() => {
                    setSelectedPeriod(p);
                    handleSelect(selectedHour, selectedMinute, p);
                  }}
                >
                  {p}
                </button>
              ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
