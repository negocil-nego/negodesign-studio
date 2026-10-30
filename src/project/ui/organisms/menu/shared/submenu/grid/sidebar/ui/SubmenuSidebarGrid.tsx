import { useState } from "react";
import type { SubmenuSidebarGridProps } from "../data/types";
import MenuTitle from "../../../../menu/menu-title/ui/MenuTitle";
import SubmenuTitle from "../../../simple/title/ui/SubmenuTitle";
import { StyleComponent } from "../../../../../../../../../components/local/data/style.component";

export default function SubmenuSidebarGrid({ title, items = [], defaultSelected = 0 }: SubmenuSidebarGridProps) {
  const [selected, setSelected] = useState(Math.min(defaultSelected, Math.max(items.length - 1, 0)));
  const active = items[selected];
  return <div className={StyleComponent.menu.submenu.sidebarGrid.root}>
    <aside className={StyleComponent.menu.submenu.sidebarGrid.sidebar}><p className={StyleComponent.menu.submenu.sidebarGrid.sidebarHeader}>Navegação</p><nav className={StyleComponent.menu.submenu.sidebarGrid.nav} aria-label="Secções do submenu">
      {items.map((item, i) => <button key={i} type="button" className={`${StyleComponent.menu.submenu.sidebarGrid.navItem} ${selected === i ? StyleComponent.menu.submenu.sidebarGrid.navItemActive : ""}`} aria-current={selected === i ? "page" : undefined} onClick={() => setSelected(i)}>{item.title ? <div className={StyleComponent.menu.text.root}>{item.title.textProps?.text}</div> : `Secção ${i + 1}`}</button>)}
    </nav></aside>
    <section className={StyleComponent.menu.submenu.sidebarGrid.content}><div className={StyleComponent.menu.submenu.sidebarGrid.contentInner}>{title && <header className={StyleComponent.menu.submenu.sidebarGrid.contentHeader}><MenuTitle {...title} variant="featured" /></header>}{active ? <div className={StyleComponent.menu.submenu.sidebarGrid.groupContent}><SubmenuTitle {...active} /></div> : <div className={StyleComponent.menu.submenu.sidebarGrid.empty}>Sem secções para apresentar.</div>}</div></section>
  </div>;
}
