import { useState } from "react";
import Icon from "../../../atoms/icon/ui/Icon";
import SidebarSection from "../shared/sidebar-section/ui/SidebarSection";
import SidebarProfile from "../shared/sidebar-profile/ui/SidebarProfile";
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
  const dark = theme === "dark",
    rail = variant === "02",
    compact = collapsed;
  const [query, setQuery] = useState("");
  return (
    <aside
      aria-label={`${brand} sidebar`}
      className={`flex h-[min(760px,90vh)] min-h-[560px] shrink-0 overflow-hidden rounded-2xl border shadow-sm ${dark ? "border-zinc-800 bg-zinc-950 text-zinc-100" : "border-zinc-200 bg-white text-zinc-900"} ${rail ? "w-[min(100%,420px)] flex-row" : compact ? "w-[76px]" : "w-[min(100%,280px)]"} ${className}`}
    >
      {rail && (
        <nav
          aria-label="Atalhos"
          className={`flex w-[68px] shrink-0 flex-col items-center gap-3 border-r p-3 ${dark ? "border-zinc-800 bg-zinc-900" : "border-zinc-100 bg-zinc-50"}`}
        >
          <b className="grid h-9 w-9 place-items-center rounded-xl bg-blue-600 text-white">
            ∞
          </b>
          <span className="my-2 h-8 w-8 rounded-full bg-blue-100 ring-2 ring-blue-500" />
          {["Grid", "Search", "Star", "Home", "Mail", "User", "Settings"].map(
            (icon, i) => (
              <button
                key={icon}
                aria-label={icon}
                className={`grid h-9 w-9 place-items-center rounded-lg ${i === 3 ? "bg-blue-600 text-white" : dark ? "text-zinc-400 hover:bg-zinc-800" : "text-zinc-500 hover:bg-zinc-200"}`}
              >
                <Icon name={icon} />
              </button>
            ),
          )}
          <div className="flex-1" />
          {["Bell", "Settings"].map((icon) => (
            <button
              key={icon}
              aria-label={icon}
              className="grid h-8 w-8 place-items-center rounded-lg text-zinc-500"
            >
              <Icon name={icon} />
            </button>
          ))}
        </nav>
      )}
      <div className="flex min-w-0 flex-1 flex-col p-4">
        <header className="mb-4 flex min-h-9 items-center gap-2">
          {!rail && (
            <span
              className={`grid h-8 w-8 shrink-0 place-items-center rounded-lg text-lg font-bold ${dark ? "bg-zinc-800" : "bg-zinc-100"}`}
            >
              {logo}
            </span>
          )}
          {!compact && (
            <div className="min-w-0 flex-1">
              <p className="truncate text-sm font-bold">{brand}</p>
              {variant === "04" && (
                <p className="truncate text-[10px] text-zinc-500">
                  {workspace}
                </p>
              )}
            </div>
          )}
          {variant === "04" && !compact && (
            <button aria-label="Mudar espaço de trabalho">⌄</button>
          )}
        </header>
        {(variant === "01" || variant === "02") && (
          <label
            className={`mb-4 flex h-9 items-center gap-2 rounded-lg border px-2.5 ${dark ? "border-zinc-700 bg-zinc-900" : "border-zinc-200 bg-white"}`}
          >
            <Icon name="Search" className="text-zinc-400" />
            <input
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              aria-label="Pesquisar"
              placeholder={compact ? "" : "Pesquisar"}
              className="w-full min-w-0 bg-transparent text-xs outline-none placeholder:text-zinc-400"
            />
            {!compact && (
              <kbd className="rounded border px-1 text-[9px] text-zinc-400">
                ⌘ K
              </kbd>
            )}
          </label>
        )}
        <div className="min-h-0 flex-1 space-y-3 overflow-y-auto">
          {sections.map((section, i) => (
            <SidebarSection
              key={`${section.title}-${i}`}
              section={section}
              compact={compact}
              dark={dark}
              accent={variant === "03" ? "violet" : "blue"}
            />
          ))}
        </div>
        {storage && !compact && (
          <div
            className={`my-3 rounded-xl border p-3 ${dark ? "border-zinc-800 bg-zinc-900" : "border-zinc-200 bg-zinc-50"}`}
          >
            <p className="text-xs font-semibold">{storage.title}</p>
            <p className="mt-1 text-[10px] leading-4 text-zinc-500">
              {storage.description}
            </p>
            <div className="mt-3 h-1.5 overflow-hidden rounded-full bg-zinc-200">
              <div
                className="h-full rounded-full bg-blue-600"
                style={{
                  width: `${Math.min(100, Math.max(0, storage.usage))}%`,
                }}
              />
            </div>
            <div className="mt-2 flex justify-between text-[10px]">
              <button>Dispensar</button>
              <button className="font-semibold text-blue-600">
                Actualizar
              </button>
            </div>
          </div>
        )}
        {!!footerSections.length && (
          <div className="space-y-1 border-t border-current/10 py-3">
            {footerSections.map((section, i) => (
              <SidebarSection
                key={`${section.title}-${i}`}
                section={section}
                compact={compact}
                dark={dark}
              />
            ))}
          </div>
        )}
        <SidebarProfile user={user} compact={compact} dark={dark} />
      </div>
    </aside>
  );
}
