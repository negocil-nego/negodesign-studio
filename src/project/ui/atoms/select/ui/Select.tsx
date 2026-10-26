// @ts-nocheck
import { useId, useState } from "react";
import Icon from "../../icon/ui/Icon";
import Popover from "../../popover/ui/Popover";
import { StyleComponent } from "../../../../data/style.component";
import type { SelectProps, SelectOption } from "../data/types";

export default function Select({ id, label, options, value = "", placeholder, disabled, required, onselect }: SelectProps) {
    const popoverId = useId();
    const [query, setQuery] = useState("");
    const [selectedValue, setSelectedValue] = useState(value ?? "");

    const selected = options.find((opt) => opt.value === selectedValue || opt.label === selectedValue);
    const term = query.trim().toLowerCase();
    const filtered = term ? options.filter((opt) => opt.label.toLowerCase().includes(term)) : options;

    function choose(option: SelectOption) {
        setSelectedValue(option.label);
        onselect?.(option);
        setQuery("");
    }

    return (
        <div className={StyleComponent.select.root}>
            <Popover
                id={popoverId}
                role="listbox"
                ariaLabel={label ?? "Select option"}
                trigger={
                    <button
                        id={id}
                        disabled={disabled}
                        type="button"
                        popoverTarget={popoverId}
                        aria-haspopup="listbox"
                        className={StyleComponent.select.trigger}
                    >
                        {label ? (
                            <div className={StyleComponent.select.triggerLabel}>{label}</div>
                        ) : (
                            <div className={StyleComponent.select.triggerValue}>
                                {selected ? selected.label : (placeholder ?? "Selecione...")}
                            </div>
                        )}
                        <div>
                            <Icon name="ArrowDown" />
                        </div>
                    </button>
                }
                panel={
                    <div className={StyleComponent.select.panel}>
                        <input
                            type="search"
                            value={query}
                            onChange={(e) => setQuery(e.target.value)}
                            placeholder="Search..."
                            className={StyleComponent.select.search}
                        />
                        <ul className={StyleComponent.select.list}>
                            {filtered.map((option) => (
                                <li key={option.value}>
                                    <button
                                        type="button"
                                        role="option"
                                        aria-selected={option.value === selectedValue || option.label === selectedValue}
                                        disabled={option.disabled}
                                        popoverTarget={popoverId}
                                        popoverTargetAction="hide"
                                        onClick={() => choose(option)}
                                        className={`${StyleComponent.select.option} ${selected?.value === option.value ? StyleComponent.select.optionSelected : ""}`}
                                    >
                                        <div className={StyleComponent.select.optionContent}>
                                            {option.icon && <span className={StyleComponent.select.optionIcon}>{option.icon}</span>}
                                            <span className="truncate">{option.label}</span>
                                        </div>
                                        {(selected?.value === option.value || selected?.label === option.label) && (
                                            <span className={StyleComponent.select.optionCheck}>✓</span>
                                        )}
                                    </button>
                                </li>
                            ))}
                            {!filtered.length && (
                                <li className={StyleComponent.select.optionEmpty}>No options found</li>
                            )}
                        </ul>
                    </div>
                }
            />
        </div>
    );
}
