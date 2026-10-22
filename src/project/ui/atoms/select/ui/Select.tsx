// @ts-nocheck
import { useId, useState } from "react";
import Icon from "../../icon/ui/Icon";
import type { SelectProps } from "../data/types";

export default function Select({ id, label, options, value, placeholder, disabled, onselect }: SelectProps) {
    const popoverId = useId();
    const [query, setQuery] = useState("");
    const [selectedValue, setSelectedValue] = useState(value ?? "");

    const selected = options.find((opt) => opt.value === selectedValue || opt.label === selectedValue);
    const term = query.trim().toLowerCase();
    const filtered = options.filter((opt) => !term || opt.label.toLowerCase().includes(term));

    return (
        <div className="relative w-full">
            <div className="popover w-full">
                <button
                    type="button"
                    id={id}
                    disabled={disabled}
                    popoverTarget={popoverId}
                    aria-haspopup="listbox"
                    className="input-form w-full cursor-pointer flex justify-between items-center text-left"
                >
                    {label ? (
                        <div className="text-[10px] uppercase font-semibold text-zinc-500 select-none w-full">
                            {label}
                        </div>
                    ) : (
                        <div className="text-sm truncate w-full">
                            {selected ? selected.label : (placeholder ?? "Selecione...")}
                        </div>
                    )}
                    <div>
                        <Icon name="Home" />
                    </div>
                </button>

                <div
                    id={popoverId}
                    popover="auto"
                    role="listbox"
                    aria-label={label ?? "Select option"}
                    className="mt-1.5 p-2 w-full min-w-60 bg-blender-card text-zinc-200 border border-blender-border rounded-lg shadow-xl z-50 outline-none"
                >
                    <input
                        type="search"
                        value={query}
                        onChange={(e) => setQuery(e.target.value)}
                        placeholder="Search..."
                        className="w-full px-3 py-1.5 mb-1.5 text-sm bg-blender-surface border border-blender-border rounded text-inherit outline-none focus:border-blue-500"
                    />
                    <ul className="max-h-56 overflow-y-auto flex flex-col gap-0.5">
                        {filtered.map((option) => (
                            <li key={option.value}>
                                <button
                                    type="button"
                                    role="option"
                                    disabled={option.disabled}
                                    popoverTarget={popoverId}
                                    popoverTargetAction="hide"
                                    onClick={() => {
                                        setSelectedValue(option.value);
                                        onselect?.(option);
                                        setQuery("");
                                    }}
                                    className={`w-full flex items-center justify-between px-3 py-2 rounded text-sm text-zinc-300 hover:text-slate-900 hover:bg-blender-hover active:bg-blender-active transition-colors cursor-pointer text-left ${selected?.value === option.value ? "bg-blender-hover text-slate-900 font-medium" : ""
                                        }`}
                                >
                                    <div className="flex items-center gap-2.5 truncate">
                                        {option.icon && <span className="text-base">{option.icon}</span>}
                                        <span className="truncate">{option.label}</span>
                                    </div>
                                    {(selected?.value === option.value || selected?.label === option.label) && (
                                        <span className="text-blue-500 font-bold text-xs">✓</span>
                                    )}
                                </button>
                            </li>
                        ))}
                        {!filtered.length && (
                            <li className="px-3 py-2 text-xs text-zinc-500 text-center select-none">
                                No options found
                            </li>
                        )}
                    </ul>
                </div>
            </div>
        </div>
    );
}