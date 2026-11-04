import { useState } from "react";
import Icon from "../../../../../atoms/icon/ui/Icon";
import type { SidebarItemProps } from "../../../data/types";
export default function SidebarItem({
  item,
  compact = false,
  dark = false,
  accent = "blue",
}: {
  item: SidebarItemProps;
  compact?: boolean;
  dark?: boolean;
  accent?: string;
}) {
  const [open, setOpen] = useState(Boolean(item.active));
  const active = dark
    ? "bg-zinc-700 text-white"
    : accent === "violet"
      ? "bg-violet-50 text-violet-700"
      : "bg-blue-50 text-blue-700";
  return (
    <div className="space-y-1">
      <a
        href={item.href ?? "#"}
        title={compact ? item.label : undefined}
        aria-current={item.active ? "page" : undefined}
        onClick={(e) => {
          if (item.children?.length) {
            e.preventDefault();
            setOpen(!open);
          }
        }}
        className={`flex min-h-9 items-center gap-3 rounded-lg px-3 text-sm transition ${compact ? "justify-center px-2" : ""} ${item.active ? active : dark ? "text-zinc-300 hover:bg-zinc-800 hover:text-white" : "text-zinc-600 hover:bg-zinc-100 hover:text-zinc-950"}`}
      >
        {item.icon && <Icon name={item.icon} />}
        {!compact && (
          <>
            <span className="min-w-0 flex-1 truncate">{item.label}</span>
            {item.badge && <small>{item.badge}</small>}
            {item.children?.length && <span>{open ? "⌃" : "⌄"}</span>}
          </>
        )}
      </a>
      {open &&
        !compact &&
        item.children?.map((child) => (
          <a
            key={child.label}
            href={child.href ?? "#"}
            className="ml-8 flex justify-between rounded-md px-2 py-2 text-xs text-zinc-500"
          >
            {child.label}
            {child.badge && <span>{child.badge}</span>}
          </a>
        ))}
    </div>
  );
}
