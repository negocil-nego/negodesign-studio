import { useState } from "react";
import Button from "../../../../atoms/button/variant/default/ui/Button";
import Icon from "../../../../atoms/icon/ui/Icon";
import Image from "../../../../atoms/image/ui/Image";
import SubmenuGridTitle from "../../shared/submenu/grid/title/ui/SubmenuGridTitle";
import SubmenuSidebarGrid from "../../shared/submenu/grid/sidebar/ui/SubmenuSidebarGrid";
import type { MenuProps } from "../data/types";
import { StyleComponent } from "../../../../../data/style.component";

export default function Menu({ logo, menus = [], buttonsProps = [], defaultOpenSubmenu = null }: MenuProps) {
  const [openSubmenu, setOpenSubmenu] = useState<number | null>(defaultOpenSubmenu);
  const active = openSubmenu === null ? undefined : menus[openSubmenu];
  const sidebar = active && "type" in active && active.type === "sidebar-grid";
  const submenu = active && (sidebar || "items" in active || "title" in active || "label" in active);
  const label = active ? (sidebar ? active.title?.title?.text : active.label ?? active.title?.title?.text) ?? "Explorar" : "";

  return <nav className={StyleComponent.menu.navigation.root} aria-label="Menu principal">
    <div className={StyleComponent.menu.navigation.bar}>
      <div className={StyleComponent.menu.navigation.logoContainer}>{logo && <Image {...logo} className={`${logo.className ?? ""} ${StyleComponent.menu.navigation.logo}`} />}</div>
      <ul className={StyleComponent.menu.navigation.list}>{menus.map((menu, index) => {
        const isSidebar = "type" in menu && menu.type === "sidebar-grid";
        const hasSubmenu = isSidebar || "items" in menu || "title" in menu || "label" in menu;
        if (hasSubmenu) return <li key={index} className={StyleComponent.menu.navigation.item}><button type="button" className={StyleComponent.menu.navigation.submenuTrigger} aria-expanded={openSubmenu === index} aria-haspopup="true" onKeyDown={e => e.key === "Escape" && setOpenSubmenu(null)} onClick={() => setOpenSubmenu(openSubmenu === index ? null : index)}><span className={StyleComponent.menu.navigation.submenuLabel}>{isSidebar ? menu.title?.title?.text ?? "Explorar" : menu.label ?? menu.title?.title?.text ?? "Explorar"}</span><Icon name={openSubmenu === index ? "ArrowUp" : "ArrowDown"} className={StyleComponent.menu.navigation.submenuIcon} /></button></li>;
        return <li key={index} className={StyleComponent.menu.navigation.item}>{menu.href ? <a href={menu.href} className={StyleComponent.menu.navigation.link}>{menu.icon && <Icon {...menu.icon} />}{menu.textProps && <span className={StyleComponent.menu.navigation.linkLabel}>{menu.textProps.text}</span>}</a> : <button type="button" className={StyleComponent.menu.navigation.link}>{menu.icon && <Icon {...menu.icon} />}{menu.textProps && <span className={StyleComponent.menu.navigation.linkLabel}>{menu.textProps.text}</span>}</button>}</li>;
      })}</ul>
      <div className={StyleComponent.menu.navigation.actions}>{buttonsProps.map((button, i) => <Button key={i} {...button} />)}</div>
    </div>
    {submenu && <div className={StyleComponent.menu.navigation.panel}>{sidebar ? <SubmenuSidebarGrid {...active} /> : <SubmenuGridTitle {...active} />}</div>}
  </nav>;
}
