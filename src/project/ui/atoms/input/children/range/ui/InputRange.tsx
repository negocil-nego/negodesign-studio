// @ts-nocheck
import { useState, useEffect } from "react";
import { StyleComponent } from "../../../../../../data/style.component";
import type { InputRangeProps } from "../data/types";

export default function InputRange({
  id,
  name,
  label = "Example range",
  value = 50,
  min = 0,
  max = 100,
  step = 1,
  disabled = false,
  showValue = true,
  hint,
  onValueChange,
}: InputRangeProps) {
  const [current, setCurrent] = useState(value);

  useEffect(() => {
    setCurrent(value);
  }, [value]);

  function handleChange(event: any) {
    const next = Number(event.target.value);
    setCurrent(next);
    onValueChange?.(next);
  }

  return (
    <div className={StyleComponent.input.range.field}>
      {(label || showValue) && (
        <div className={StyleComponent.input.range.header}>
          {label && (
            <label htmlFor={id} className={StyleComponent.input.range.label}>
              {label}
            </label>
          )}
          {showValue && (
            <span className={StyleComponent.input.range.valueBadge}>{current}</span>
          )}
        </div>
      )}
      <input
        type="range"
        id={id}
        name={name}
        min={min}
        max={max}
        step={step}
        disabled={disabled}
        value={current}
        onChange={handleChange}
        className={StyleComponent.input.range.input}
      />
      {hint && <span className={StyleComponent.input.range.hint}>{hint}</span>}
    </div>
  );
}
