// @ts-nocheck
import { useId, useState } from "react";
import Icon from "../../../../icon/ui/Icon";
import Popover from "../../../../popover/ui/Popover";
import { StyleComponent } from "../../../../../../data/style.component";
import { useI18n } from "../../../../../../data/i18n/react";
import {
  computeCalendarMonth,
  formatLongDate,
  stepCalendarMonth,
} from "../../../../../../hooks/date-picker.hook";
import type { InputDateProps } from "../data/types";

export default function InputDate({ labelProps, inputProps, value, onselect }: InputDateProps & {
  value?: string;
  onselect?: (date: string) => void;
}) {
  const popoverId = useId();
  const { locale, t } = useI18n();
  const style = StyleComponent.input.date;
  const [year, setYear] = useState(2023);
  const [month, setMonth] = useState(10);
  const [day, setDay] = useState(21);
  const [dateStr, setDateStr] = useState(value ?? formatLongDate(locale, 2023, 10, 21));

  const cal = computeCalendarMonth(locale, year, month);

  function changeMonth(delta: number) {
    const next = stepCalendarMonth(year, month, delta);
    setYear(next.year);
    setMonth(next.month);
  }

  function handleSelect(selectedDay: number) {
    setDay(selectedDay);
    const nextVal = formatLongDate(locale, year, month, selectedDay);
    setDateStr(nextVal);
    onselect?.(nextVal);
  }

  return (
    <div className={style.root}>
      <Popover
        id={popoverId}
        role="dialog"
        ariaLabel={labelProps?.label ?? t("input.date.selectDate")}
        trigger={
          <button type="button" id={inputProps?.id} disabled={inputProps?.disabled} popoverTarget={popoverId}
            aria-haspopup="dialog" className={style.trigger}>
            <div className={style.triggerBody}>
              {labelProps && <span className={style.triggerLabel}>{labelProps.label}</span>}
              <div className={style.valueRow}>
                <span className={style.valueIcon}><Icon name="Calendar" /></span>
                <span className={style.value}>{dateStr}</span>
              </div>
            </div>
          </button>
        }
        panel={
          <div className={style.panel}>
            <div className={style.panelHeader}>
              <button type="button" onClick={() => changeMonth(-1)} className={style.navButton}><Icon name="ArrowLeft" /></button>
              <span className={style.monthTitle}>{cal.monthName} {year}</span>
              <button type="button" onClick={() => changeMonth(1)} className={style.navButton}><Icon name="ArrowRight" /></button>
            </div>
            <div className={style.weekdays}>
              {cal.weekdays.map((wd) => <span key={wd}>{wd}</span>)}
            </div>
            <div className={style.daysGrid}>
              {cal.paddingDays.map((pad, idx) => <span key={`p-${idx}`} className={style.paddingDay}>{pad}</span>)}
              {cal.days.map((d) => (
                <button key={d} type="button" popoverTarget={popoverId} popoverTargetAction="hide"
                  onClick={() => handleSelect(d)}
                  className={`${style.day} ${d === day ? style.daySelected : style.dayDefault}`}>{d}</button>
              ))}
            </div>
          </div>
        }
      />
    </div>
  );
}
