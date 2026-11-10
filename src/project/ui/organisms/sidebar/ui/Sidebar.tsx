import { useState } from "react";
import Icon from "../../../atoms/icon/ui/Icon";
import SidebarSection from "../shared/sidebar-section/ui/SidebarSection";
import SidebarProfile from "../shared/sidebar-profile/ui/SidebarProfile";
import {
  computeSidebarTheme,
  SIDEBAR_RAIL_FOOTER_ICONS,
  SIDEBAR_RAIL_ICONS,
} from "../../../../hooks/sidebar.hook";
import type { SidebarProps } from "../data/types";

export default function Sidebar({
  variant = "01",
  theme = "light",
  collapsed = false,
  logo = "◈",
  brand = "Untitled UI",
  workspace = "workspace.untitledui.com",
  sections = [],
  footerSections = [],
  user = { name: "Sienna Hewitt", detail: "sienna@untitledui.com" },
  storage,
  className = "",
}: SidebarProps) {
  const cfg = computeSidebarTheme(variant, theme, collapsed, className);
  const [query, setQuery] = useState("");

  return (
    <aside aria-label={`${brand} sidebar`} className={cfg.rootClass}>
      {cfg.rail && (
        <nav aria-label="Shortcuts" className={cfg.railClass}>
          <b className="grid h-9 w-9 place-items-center rounded-xl bg-blue-600 text-white">∞</b>
          <span className="my-2 h-8 w-8 rounded-full bg-blue-100 ring-2 ring-blue-500" />
          {SIDEBAR_RAIL_ICONS.map((icon, i) => (
            <button key={icon} aria-label={icon}
              className={`grid h-9 w-9 place-items-center rounded-lg ${i === 3 ? "bg-blue-600 text-white" : cfg.dark ? "text-zinc-400 hover:bg-zinc-800" : "text-zinc-500 hover:bg-zinc-200"}`}>
              <Icon name={icon} />
            </button>
          ))}
          <div className="flex-1" />
          {SIDEBAR_RAIL_FOOTER_ICONS.map((icon) => (
            <button key={icon} aria-label={icon} className="grid h-8 w-8 place-items-center rounded-lg text-zinc-500">
              <Icon name={icon} />
            </button>
          ))}
        </nav>
      )}
      <div className="flex min-w-0 flex-1 flex-col p-4">
        <header className="mb-4 flex min-h-9 items-center gap-2">
          {!cfg.rail && <span className={cfg.logoBoxClass}>{logo}</span>}
          {!cfg.compact && (
            <div className="min-w-0 flex-1">
              <p className="truncate text-sm font-bold">{brand}</p>
              {variant === "04" && <p className="truncate text-[10px] text-zinc-500">{workspace}</p>}
            </div>
          )}
          {variant === "04" && !cfg.compact && <button aria-label="Switch workspace">⌄</button>}
        </header>
        {cfg.showSearch && (
          <label className={cfg.searchClass}>
            <Icon name="Search" className="text-zinc-400" />
            <input value={query} onChange={(e) => setQuery(e.target.value)} aria-label="Search"
              placeholder={cfg.compact ? "" : "Search"} className="w-full min-w-0 bg-transparent text-xs outline-none placeholder:text-zinc-400" />
            {!cfg.compact && <kbd className="rounded border px-1 text-[9px] text-zinc-400">⌘ K</kbd>}
          </label>
        )}
        <div className="min-h-0 flex-1 space-y-3 overflow-y-auto">
          {sections.map((section, i) => (
            <SidebarSection key={`${section.title}-${i}`} section={section} compact={cfg.compact} dark={cfg.dark} accent={cfg.accent} />
          ))}
        </div>
        {storage && !cfg.compact && (
          <div className={cfg.storageClass}>
            <p className="text-xs font-semibold">{storage.title}</p>
            <p className="mt-1 text-[10px] leading-4 text-zinc-500">{storage.description}</p>
            <div className="mt-3 h-1.5 overflow-hidden rounded-full bg-zinc-200">
              <div className="h-full rounded-full bg-blue-600" style={{ width: `${Math.min(100, Math.max(0, storage.usage))}%` }} />
            </div>
          </div>
        )}
        {!!footerSections.length && (
          <div className="space-y-1 border-t border-current/10 py-3">
            {footerSections.map((section, i) => (
              <SidebarSection key={`${section.title}-${i}`} section={section} compact={cfg.compact} dark={cfg.dark} />
            ))}
          </div>
        )}
        <SidebarProfile user={user} compact={cfg.compact} dark={cfg.dark} />
      </div>
    </aside>
  );
}
