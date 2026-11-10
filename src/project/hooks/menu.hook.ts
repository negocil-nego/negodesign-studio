import { LocalStyleComponent } from "../data/style.component";
import type { MenuTypeProps } from "../ui/organisms/menu/simple/data/types";
import type { SubmenuGridTitleProps } from "../ui/organisms/menu/shared/submenu/grid/title/data/types";
import type { SubmenuSidebarGridProps } from "../ui/organisms/menu/shared/submenu/grid/sidebar/data/types";

export function isSidebarSubmenu(menu: MenuTypeProps): menu is SubmenuSidebarGridProps {
  return "type" in menu && menu.type === "sidebar-grid";
}

export function isGridSubmenu(menu: MenuTypeProps): menu is SubmenuGridTitleProps {
  return !isSidebarSubmenu(menu) && ("items" in menu || "title" in menu || "label" in menu);
}

export function isSubmenu(
  menu: MenuTypeProps,
): menu is SubmenuGridTitleProps | SubmenuSidebarGridProps {
  return isSidebarSubmenu(menu) || isGridSubmenu(menu);
}

export function getSubmenuLabel(menu: SubmenuGridTitleProps | SubmenuSidebarGridProps): string {
  return isSidebarSubmenu(menu)
    ? (menu.title?.title?.text ?? "Explore")
    : (menu.label ?? menu.title?.title?.text ?? "Explore");
}

export function resolveMenuClasses(style = "default") {
  const nav = LocalStyleComponent.menu.navigation;
  const isGhost = style === "ghost" || style === "ghost-inverse";
  const isInverse = style === "ghost-inverse";
  return {
    nav,
    isGhost,
    root: isGhost ? nav.ghostRoot : nav.root,
    bar: isGhost ? nav.ghostBar : nav.bar,
    brand: isInverse ? nav.ghostInverseBrand : nav.ghostBrand,
    brandSubtitle: isInverse ? nav.ghostInverseBrandSubtitle : nav.ghostBrandSubtitle,
    list: isGhost ? nav.ghostList : nav.list,
    submenuTrigger: isInverse
      ? nav.ghostInverseSubmenuTrigger
      : style === "ghost"
        ? nav.ghostSubmenuTrigger
        : nav.submenuTrigger,
    link: isInverse ? nav.ghostInverseLink : style === "ghost" ? nav.ghostLink : nav.link,
    actions: isGhost ? nav.ghostActions : nav.actions,
    buttonClass: (custom?: string) =>
      isInverse ? nav.ghostInverseButton : style === "ghost" ? nav.ghostButton : custom,
  };
}
