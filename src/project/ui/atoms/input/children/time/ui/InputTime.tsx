// @ts-nocheck
import { useEffect, useId, useState } from "react";
import Icon from "../../../../icon/ui/Icon";
import Popover from "../../../../popover/ui/Popover";
import { StyleComponent } from "../../../../../../data/style.component";
import { useI18n } from "../../../../../../data/i18n/react";
import { formatTimeDisplay, parseTimeValue, TIME_HOURS, TIME_MINUTES, TIME_PERIODS } from "../../../../../../hooks/time-picker.hook";
import type { InputTimeProps } from "../data/types";

export default function InputTime({ labelProps, inputProps, value = "", onselect }: InputTimeProps & {
  value?: string;
  onselect?: (time: string) => void;
}) {
  const popoverId = useId();
  const { t } = useI18n();
  const style = StyleComponent.input.time;
  const [selectedHour, setSelectedHour] = useState("02");
  const [selectedMinute, setSelectedMinute] = useState("30");
  const [selectedPeriod, setSelectedPeriod] = useState("AM");
  const [currentTime, setCurrentTime] = useState(value);

  useEffect(() => {
    if (value) {
      const parsed = parseTimeValue(value);
      setSelectedHour(parsed.hour);
      setSelectedMinute(parsed.minute);
      setSelectedPeriod(parsed.period);
      setCurrentTime(value);
    }
  }, [value]);

  const displayTime = formatTimeDisplay(selectedHour, selectedMinute, selectedPeriod);

  function apply(h: string, m: string, p: string) {
    setSelectedHour(h);
    setSelectedMinute(m);
    setSelectedPeriod(p);
    const nextVal = formatTimeDisplay(h, m, p);
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

  const columns: [readonly string[], string, (v: string) => void][] = [
    [TIME_HOURS, selectedHour, (h) => apply(h, selectedMinute, selectedPeriod)],
    [TIME_MINUTES, selectedMinute, (m) => apply(selectedHour, m, selectedPeriod)],
    [TIME_PERIODS, selectedPeriod, (p) => apply(selectedHour, selectedMinute, p)],
  ];

  return (
    <div className={style.root}>
      <Popover
        id={popoverId}
        role="dialog"
        ariaLabel={labelProps?.label ?? t("input.time.selectTime")}
        trigger={
          <button type="button" id={inputProps?.id} disabled={inputProps?.disabled} popoverTarget={popoverId} aria-haspopup="dialog" className={style.trigger}>
            <div className={style.triggerBody}>
              {labelProps && <span className={style.triggerLabel}>{labelProps.label}</span>}
              <div className={style.valueRow}><span className={style.valueIcon}><Icon name="Time" /></span><span className={style.value}>{displayTime}</span></div>
            </div>
            {currentTime && <button type="button" className={style.clearButton} onClick={clearTime} aria-label={t("input.time.clear")}><Icon name="Cancel" /></button>}
          </button>
        }
        panel={
          <div className={style.panel}>
            <div className={style.pickerGrid}>
              {columns.map(([items, active, onPick], idx) => (
                <div key={idx} className="contents">
                  {idx > 0 && <div className={style.columnDivider} />}
                  <div className={style.pickerColumn}>
                    {items.map((item) => (
                      <button key={item} type="button" className={`${style.pickerItem} ${active === item ? style.pickerItemSelected : ""}`} onClick={() => onPick(item)}>{item}</button>
                    ))}
                  </div>
                </div>
              ))}
            </div>
          </div>
        }
      />
    </div>
  );
}
