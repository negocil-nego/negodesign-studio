import { useState } from "react";
import Button from "../../../../atoms/button/variant/default/ui/Button";
import Icon from "../../../../atoms/icon/ui/Icon";
import Image from "../../../../atoms/image/ui/Image";
import SubmenuGridTitle from "../../shared/submenu/grid/title/ui/SubmenuGridTitle";
import SubmenuSidebarGrid from "../../shared/submenu/grid/sidebar/ui/SubmenuSidebarGrid";
import type { MenuProps } from "../data/types";
import { StyleComponent } from "../../../../../data/style.component";
import { getSubmenuLabel, isSidebarSubmenu, isSubmenu } from "../../../../../hooks/menu.hook";

export default function Menu({
  logo,
  menus = [],
  buttonsProps = [],
  defaultOpenSubmenu = null,
}: MenuProps) {
  const [openSubmenu, setOpenSubmenu] = useState<number | null>(defaultOpenSubmenu);
  const active = openSubmenu === null ? undefined : menus[openSubmenu];
  const nav = StyleComponent.menu.navigation;

  return (
    <nav className={nav.root} aria-label="Menu principal">
      <div className={nav.bar}>
        <div className={nav.logoContainer}>
          {logo && <Image {...logo} className={`${logo.className ?? ""} ${nav.logo}`} />}
        </div>
        <ul className={nav.list}>
          {menus.map((menu, index) => {
            if (isSubmenu(menu)) {
              const isOpen = openSubmenu === index;
              return (
                <li key={index} className={nav.item}>
                  <button
                    type="button"
                    className={nav.submenuTrigger}
                    aria-expanded={isOpen}
                    aria-haspopup="true"
                    onKeyDown={(e) => e.key === "Escape" && setOpenSubmenu(null)}
                    onClick={() => setOpenSubmenu(isOpen ? null : index)}
                  >
                    <span className={nav.submenuLabel}>{getSubmenuLabel(menu)}</span>
                    <Icon name={isOpen ? "ArrowUp" : "ArrowDown"} className={nav.submenuIcon} />
                  </button>
                </li>
              );
            }
            return (
              <li key={index} className={nav.item}>
                {menu.href ? (
                  <a href={menu.href} className={nav.link}>
                    {menu.icon && <Icon {...menu.icon} />}
                    {menu.textProps && <span className={nav.linkLabel}>{menu.textProps.text}</span>}
                  </a>
                ) : (
                  <button type="button" className={nav.link}>
                    {menu.icon && <Icon {...menu.icon} />}
                    {menu.textProps && <span className={nav.linkLabel}>{menu.textProps.text}</span>}
                  </button>
                )}
              </li>
            );
          })}
        </ul>
        <div className={nav.actions}>
          {buttonsProps.map((button, i) => (
            <Button key={i} {...button} />
          ))}
        </div>
      </div>
      {active && isSubmenu(active) && (
        <div className={nav.panel}>
          {isSidebarSubmenu(active) ? (
            <SubmenuSidebarGrid {...active} />
          ) : (
            <SubmenuGridTitle {...active} />
          )}
        </div>
      )}
    </nav>
  );
}
