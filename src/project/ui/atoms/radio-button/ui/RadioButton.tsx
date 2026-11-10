// @ts-nocheck
import { useState, useEffect } from "react";
import { StyleComponent } from "../../../../data/style.component";
import type { RadioButtonProps } from "../data/types";

export default function RadioButton({
  id,
  name = "btnradio",
  options = [],
  value = "",
  multiple = false,
  values = [],
  disabled = false,
  onValueChange,
  onValuesChange,
}: RadioButtonProps) {
  const [selected, setSelected] = useState(value);
  const [selectedList, setSelectedList] = useState<string[]>(values);

  useEffect(() => {
    setSelected(value);
  }, [value]);

  function handleSelect(optionValue: string) {
    if (disabled) return;
    if (multiple) {
      const next = selectedList.includes(optionValue)
        ? selectedList.filter((item) => item !== optionValue)
        : [...selectedList, optionValue];
      setSelectedList(next);
      onValuesChange?.(next);
    } else {
      setSelected(optionValue);
      onValueChange?.(optionValue);
    }
  }

  return (
    <div className={StyleComponent.radioButton.group} role="group" aria-label={name}>
      {options.map((option, idx) => {
        const active = multiple ? selectedList.includes(option.value) : selected === option.value;
        const isDisabled = disabled || Boolean(option.disabled);
        return (
          <label
            key={option.value}
            htmlFor={id ? `${id}-${idx}` : undefined}
            className={`${StyleComponent.radioButton.item} ${
              active ? StyleComponent.radioButton.active : StyleComponent.radioButton.inactive
            } ${isDisabled ? StyleComponent.radioButton.disabled : ""}`}
          >
            <input
              id={id ? `${id}-${idx}` : undefined}
              type={multiple ? "checkbox" : "radio"}
              name={name}
              value={option.value}
              checked={active}
              disabled={isDisabled}
              onChange={() => handleSelect(option.value)}
              className={StyleComponent.radioButton.input}
            />
            <span>{option.label}</span>
          </label>
        );
      })}
    </div>
  );
}
