import { useId, useState } from "react";
import type { ComponentPropsWithoutRef, MouseEvent } from "react";
import Icon from "../../icon/ui/Icon";
import Popover from "../../popover/ui/Popover";
import { StyleComponent } from "../../../../data/style.component";
import type { SelectProps, SelectOption } from "../data/types";

type Props = SelectProps & Omit<ComponentPropsWithoutRef<"button">, keyof SelectProps | "type">;

export default function Select({
  id, name, label, options, value = "", placeholder, disabled, required, onOptionSelect, onClick, className, ...rest
}: Props) {
  const popoverId = useId();
  const [query, setQuery] = useState("");
  const [selectedValue, setSelectedValue] = useState(value ?? "");

  const selected = options.find((opt) => opt.value === selectedValue || opt.label === selectedValue);
  const term = query.trim().toLowerCase();
  const filtered = term ? options.filter((opt) => opt.label.toLowerCase().includes(term)) : options;
  const s = StyleComponent.select;

  function choose(option: SelectOption) {
    setSelectedValue(option.value);
    onOptionSelect?.(option);
    setQuery("");
  }

  return (
    <div className={s.root}>
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
            onClick={(event: MouseEvent<HTMLButtonElement>) => onClick?.(event)}
            className={`${s.trigger} ${className ?? ""}`}
          >
            {label ? (
              <div className={s.triggerLabel}>{label}</div>
            ) : (
              <div className={s.triggerValue}>{selected ? selected.label : (placeholder ?? "Selecione...")}</div>
            )}
            <div><Icon name="ArrowDown" /></div>
          </button>
        }
        panel={
          <div className={s.panel}>
            <input
              type="search"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Search..."
              className={s.search}
            />
            <ul className={s.list}>
              {filtered.map((option) => {
                const isSelected = selected?.value === option.value || selected?.label === option.label;
                return (
                  <li key={option.value}>
                    <button
                      type="button"
                      role="option"
                      aria-selected={isSelected}
                      disabled={option.disabled}
                      popoverTarget={popoverId}
                      popoverTargetAction="hide"
                      onClick={() => choose(option)}
                      className={`${s.option} ${isSelected ? s.optionSelected : ""}`}
                    >
                      <div className={s.optionContent}>
                        {option.icon && <span className={s.optionIcon}>{option.icon}</span>}
                        <span className="truncate">{option.label}</span>
                      </div>
                      {isSelected && <span className={s.optionCheck}>✓</span>}
                    </button>
                  </li>
                );
              })}
              {!filtered.length && <li className={s.optionEmpty}>No options found</li>}
            </ul>
          </div>
        }
      />
      <input type="hidden" name={name} value={selected?.value ?? ""} />
    </div>
  );
}
