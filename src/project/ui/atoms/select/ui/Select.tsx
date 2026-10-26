import { useId, useState } from "react";
import type { ComponentPropsWithoutRef, MouseEvent } from "react";
import Icon from "../../icon/ui/Icon";
import Popover from "../../popover/ui/Popover";
import { StyleComponent } from "../../../../data/style.component";
import type { SelectProps, SelectOption } from "../data/types";

type Props = SelectProps & Omit<ComponentPropsWithoutRef<"button">, keyof SelectProps | "type">;

export default function Select({ id, name, label, options, value = "", placeholder, disabled, required, onOptionSelect, onClick, className, ...rest }: Props) {
    const popoverId = useId();
    const [query, setQuery] = useState("");
    const [selectedValue, setSelectedValue] = useState(value ?? "");

    const selected = options.find((opt) => opt.value === selectedValue || opt.label === selectedValue);
    const term = query.trim().toLowerCase();
    const filtered = term ? options.filter((opt) => opt.label.toLowerCase().includes(term)) : options;

    function choose(option: SelectOption) {
        setSelectedValue(option.value);
        onOptionSelect?.(option);
        setQuery("");
    }

    function handleClick(event: MouseEvent<HTMLButtonElement>) {
        onClick?.(event);
    }

    return (
        <div className={StyleComponent.select.root}>
            <Popover
                id={popoverId}
                role="listbox"
                ariaLabel={label ?? "Select option"}
                trigger={
                    <button
                        {...rest}
                        id={id}
                        disabled={disabled}
                        name={name}
                        type="button"
                        popoverTarget={popoverId}
                        aria-haspopup="listbox"
                        aria-required={required}
                        onClick={handleClick}
                        className={`${StyleComponent.select.trigger} ${className ?? ""}`}
                    >
                        {label ? <div className={StyleComponent.select.triggerLabel}>{label}</div> : <div className={StyleComponent.select.triggerValue}>{selected ? selected.label : (placeholder ?? "Selecione...")}</div>}
                        <div><Icon name="ArrowDown" /></div>
                    </button>
                }
                panel={
                    <div className={StyleComponent.select.panel}>
                        <input type="search" value={query} onChange={(e) => setQuery(e.target.value)} placeholder="Search..." className={StyleComponent.select.search} />
                        <ul className={StyleComponent.select.list}>
                            {filtered.map((option) => <li key={option.value}>
                                <button type="button" role="option" aria-selected={option.value === selectedValue || option.label === selectedValue} disabled={option.disabled} popoverTarget={popoverId} popoverTargetAction="hide" onClick={() => choose(option)} className={`${StyleComponent.select.option} ${selected?.value === option.value ? StyleComponent.select.optionSelected : ""}`}>
                                    <div className={StyleComponent.select.optionContent}>
                                        {option.icon && <span className={StyleComponent.select.optionIcon}>{option.icon}</span>}
                                        <span className="truncate">{option.label}</span>
                                    </div>
                                    {(selected?.value === option.value || selected?.label === option.label) && <span className={StyleComponent.select.optionCheck}>✓</span>}
                                </button>
                            </li>)}
                            {!filtered.length && <li className={StyleComponent.select.optionEmpty}>No options found</li>}
                        </ul>
                    </div>
                }
            />
            <input type="hidden" name={name} value={selected?.value ?? ""} />
        </div>
    );
}