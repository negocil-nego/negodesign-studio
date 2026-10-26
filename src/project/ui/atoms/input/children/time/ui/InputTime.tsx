// @ts-nocheck
import { useId, useState, useEffect } from "react";
import Icon from "../../../../icon/ui/Icon";
import Popover from "../../../../popover/ui/Popover";
import { StyleComponent } from "../../../../../../data/style.component";
import { useI18n } from "../../../../../../data/i18n/react";
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
    const { t } = useI18n();
    const style = StyleComponent.input.time;
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
        <div className={style.root}>
            <Popover
                id={popoverId}
                role="dialog"
                ariaLabel={labelProps?.label ?? t("input.time.selectTime")}
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
                                    <Icon name="Time" />
                                </span>
                                <span className={style.value}>{displayTime}</span>
                            </div>
                        </div>

                        <div className="flex items-center gap-1">
                            {currentTime && (
                                <button
                                    type="button"
                                    className={style.clearButton}
                                    onClick={clearTime}
                                    aria-label={t("input.time.clear")}
                                >
                                    <Icon name="Cancel" />
                                </button>
                            )}
                        </div>
                    </button>
                }
                panel={
                    <div className={style.panel}>
                        <div className={style.pickerGrid}>
                            {/* Hours */}
                            <div className={style.pickerColumn}>
                                {hours.map((h) => (
                                    <button
                                        key={h}
                                        type="button"
                                        className={`${style.pickerItem} ${selectedHour === h ? style.pickerItemSelected : ""}`}
                                        onClick={() => {
                                            setSelectedHour(h);
                                            handleSelect(h, selectedMinute, selectedPeriod);
                                        }}
                                    >
                                        {h}
                                    </button>
                                ))}
                            </div>

                            <div className={style.columnDivider}></div>

                            {/* Minutes */}
                            <div className={style.pickerColumn}>
                                {minutes.map((m) => (
                                    <button
                                        key={m}
                                        type="button"
                                        className={`${style.pickerItem} ${selectedMinute === m ? style.pickerItemSelected : ""}`}
                                        onClick={() => {
                                            setSelectedMinute(m);
                                            handleSelect(selectedHour, m, selectedPeriod);
                                        }}
                                    >
                                        {m}
                                    </button>
                                ))}
                            </div>

                            <div className={style.columnDivider}></div>

                            {/* Period */}
                            <div className={style.pickerColumn}>
                                {periods.map((p) => (
                                    <button
                                        key={p}
                                        type="button"
                                        className={`${style.pickerItem} ${selectedPeriod === p ? style.pickerItemSelected : ""}`}
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
                }
            />
        </div>
    );
}
