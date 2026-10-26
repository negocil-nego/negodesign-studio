// @ts-nocheck
import { useId, useState } from "react";
import Icon from "../../../../icon/ui/Icon";
import Popover from "../../../../popover/ui/Popover";
import { StyleComponent } from "../../../../../../data/style.component";
import { useI18n } from "../../../../../../data/i18n/react";
import type { InputDateProps } from "../data/types";

export default function InputDate({
    labelProps,
    inputProps,
    value,
    onselect,
}: InputDateProps & {
    value?: string;
    onselect?: (date: string) => void;
}) {
    const popoverId = useId();
    const { locale, t } = useI18n();
    const style = StyleComponent.input.date;
    const [year, setYear] = useState(2023);
    const [month, setMonth] = useState(10); // 10 = November
    const [day, setDay] = useState(21);
    const [dateStr, setDateStr] = useState(
        value ?? new Intl.DateTimeFormat(locale, { dateStyle: "long" }).format(new Date(2023, 10, 21)),
    );
    const monthName = new Intl.DateTimeFormat(locale, { month: "long" }).format(new Date(year, month, 1));
    const weekdays = Array.from({ length: 7 }, (_, index) =>
        new Intl.DateTimeFormat(locale, { weekday: "short" }).format(new Date(2023, 0, index + 2)),
    );
    const daysInMonth = new Date(year, month + 1, 0).getDate();
    const startDay = (new Date(year, month, 1).getDay() + 6) % 7;
    const days = Array.from({ length: daysInMonth }, (_, i) => i + 1);
    const prevDaysCount = new Date(year, month, 0).getDate();
    const paddingDays = Array.from(
        { length: startDay },
        (_, i) => prevDaysCount - startDay + i + 1,
    );

    function changeMonth(delta: number) {
        const next = month + delta;
        if (next > 11) {
            setMonth(0);
            setYear(year + 1);
        } else if (next < 0) {
            setMonth(11);
            setYear(year - 1);
        } else {
            setMonth(next);
        }
    }

    function handleSelect(selectedDay: number) {
        setDay(selectedDay);
        const nextVal = new Intl.DateTimeFormat(locale, { dateStyle: "long" }).format(
            new Date(year, month, selectedDay),
        );
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
                    <button
                        type="button"
                        id={inputProps?.id}
                        disabled={inputProps?.disabled}
                        popoverTarget={popoverId}
                        aria-haspopup="dialog"
                        className={style.trigger}
                    >
                        <div className={style.triggerBody}>
                            {labelProps && (
                                <span className={style.triggerLabel}>
                                    {labelProps.label}
                                </span>
                            )}
                            <div className={style.valueRow}>
                                <span className={style.valueIcon}>
                                    <Icon name="Calendar" />
                                </span>
                                <span className={style.value}>{dateStr}</span>
                            </div>
                        </div>
                    </button>
                }
                panel={
                    <div className={style.panel}>
                        <div className={style.panelHeader}>
                            <button
                                type="button"
                                onClick={() => changeMonth(-1)}
                                className={style.navButton}
                            >
                                <Icon name="ArrowLeft" />
                            </button>
                            <span className={style.monthTitle}>{monthName} {year}</span>
                            <button
                                type="button"
                                onClick={() => changeMonth(1)}
                                className={style.navButton}
                            >
                                <Icon name="ArrowRight" />
                            </button>
                        </div>

                        <div className={style.weekdays}>
                            {weekdays.map((weekday) => <span key={weekday}>{weekday}</span>)}
                        </div>

                        <div className={style.daysGrid}>
                            {paddingDays.map((pad, idx) => (
                                <span key={`pad-${idx}`} className={style.paddingDay}>{pad}</span>
                            ))}
                            {days.map((d) => (
                                <button
                                    key={d}
                                    type="button"
                                    popoverTarget={popoverId}
                                    popoverTargetAction="hide"
                                    onClick={() => handleSelect(d)}
                                    className={`${style.day} ${d === day ? style.daySelected : style.dayDefault}`}
                                >
                                    {d}
                                </button>
                            ))}
                        </div>
                    </div>
                }
            />
        </div>
    );
}
