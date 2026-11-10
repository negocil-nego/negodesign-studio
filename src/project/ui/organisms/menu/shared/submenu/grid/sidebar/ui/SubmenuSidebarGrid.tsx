import { useState } from "react";
import type { SubmenuSidebarGridProps } from "../data/types";
import MenuTitle from "../../../../menu/menu-title/ui/MenuTitle";
import SubmenuTitle from "../../../simple/title/ui/SubmenuTitle";
import { StyleComponent } from "../../../../../../../../data/style.component";

export default function SubmenuSidebarGrid({
  title,
  items = [],
  defaultSelected = 0,
}: SubmenuSidebarGridProps) {
  const [selected, setSelected] = useState(
    Math.min(defaultSelected, Math.max(items.length - 1, 0))
  );
  const active = items[selected];
  const s = StyleComponent.menu.submenu.sidebarGrid;

  return (
    <div className={s.root}>
      <aside className={s.sidebar}>
        <p className={s.sidebarHeader}>Navegação</p>
        <nav className={s.nav} aria-label="Secções do submenu">
          {items.map((item, i) => (
            <button
              key={i}
              type="button"
              className={`${s.navItem} ${selected === i ? s.navItemActive : ""}`}
              aria-current={selected === i ? "page" : undefined}
              onClick={() => setSelected(i)}
            >
              {item.title ? (
                <div className={StyleComponent.menu.text.root}>{item.title.textProps?.text}</div>
              ) : (
                `Secção ${i + 1}`
              )}
            </button>
          ))}
        </nav>
      </aside>
      <section className={s.content}>
        <div className={s.contentInner}>
          {title && (
            <header className={s.contentHeader}>
              <MenuTitle {...title} variant="featured" />
            </header>
          )}
          {active ? (
            <div className={s.groupContent}>
              <SubmenuTitle {...active} />
            </div>
          ) : (
            <div className={s.empty}>Sem secções para apresentar.</div>
          )}
        </div>
      </section>
    </div>
  );
}
