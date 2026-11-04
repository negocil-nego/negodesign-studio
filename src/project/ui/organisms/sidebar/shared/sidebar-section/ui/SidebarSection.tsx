import SidebarItem from "../../sidebar-item/ui/SidebarItem";
import type { SidebarSectionProps } from "../../../data/types";
export default function SidebarSection({
  section,
  compact = false,
  dark = false,
  accent = "blue",
}: {
  section: SidebarSectionProps;
  compact?: boolean;
  dark?: boolean;
  accent?: string;
}) {
  return (
    <section className="space-y-1.5">
      {section.title && !compact && (
        <h3
          className={`px-3 pb-1 pt-3 text-[10px] font-semibold uppercase tracking-[0.14em] ${dark ? "text-zinc-500" : "text-zinc-400"}`}
        >
          {section.title}
        </h3>
      )}
      <div className="space-y-1">
        {section.items.map((item) => (
          <SidebarItem
            key={item.label}
            item={item}
            compact={compact}
            dark={dark}
            accent={accent}
          />
        ))}
      </div>
    </section>
  );
}
