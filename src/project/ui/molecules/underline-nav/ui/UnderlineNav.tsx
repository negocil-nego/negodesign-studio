// @ts-nocheck
import { useState, useEffect } from "react";
import { StyleComponent } from "../../../../data/style.component";
import type { UnderlineNavProps } from "../data/types";

export default function UnderlineNav({
  items = [
    { value: "active", label: "Active" },
    { value: "link-1", label: "Link" },
    { value: "link-2", label: "Link" },
    { value: "disabled", label: "Disabled", disabled: true },
  ],
  value = "active",
  onValueChange,
}: UnderlineNavProps) {
  const [activeValue, setActiveValue] = useState(value);

  useEffect(() => {
    setActiveValue(value);
  }, [value]);

  function select(next: string, disabled?: boolean) {
    if (disabled) return;
    setActiveValue(next);
    onValueChange?.(next);
  }

  return (
    <ul className={StyleComponent.underlineNav.root} role="tablist">
      {items.map((item) => {
        const active = item.value === activeValue;
        return (
          <li key={item.value} className={StyleComponent.underlineNav.item}>
            <button
              type="button"
              role="tab"
              aria-selected={active}
              disabled={item.disabled}
              onClick={() => select(item.value, item.disabled)}
              className={`${StyleComponent.underlineNav.link} ${
                active ? StyleComponent.underlineNav.linkActive : ""
              } ${item.disabled ? StyleComponent.underlineNav.linkDisabled : ""}`}
            >
              {item.label}
            </button>
          </li>
        );
      })}
    </ul>
  );
}
