// @ts-nocheck
import { useId, useState } from "react";
import Icon from "../../../../icon/ui/Icon";
import Popover from "../../../../popover/ui/Popover";
import { StyleComponent } from "../../../../../../data/style.component";
import type { InputDateProps } from "../data/types";

export default function InputDate({
    labelProps,
    inputProps,
    value = "November 21, 2023",
    onselect,
}: InputDateProps & {
    value?: string;
    onselect?: (date: string) => void;
}) {
    const popoverId = useId();
    const style = StyleComponent.input.date;
    const [year, setYear] = useState(2023);
    const [month, setMonth] = useState(10); // 10 = November
    const [day, setDay] = useState(21);
    const [dateStr, setDateStr] = useState(value);

    const months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    const monthName = months[month];
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
        const nextVal = `${monthName} ${selectedDay}, ${year}`;
        setDateStr(nextVal);
        onselect?.(nextVal);
    }

    return (
        <div className={style.root}>
            <Popover
                id={popoverId}
                role="dialog"
                ariaLabel={labelProps?.label ?? "Pick a date"}
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
                            <span>Mon</span><span>Tue</span><span>Wed</span><span>Thu</span><span>Fri</span><span>Sat</span><span>Sun</span>
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
