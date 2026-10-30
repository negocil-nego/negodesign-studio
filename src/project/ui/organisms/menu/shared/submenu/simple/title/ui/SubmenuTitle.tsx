import type { SubmenuTitleProps } from "../data/types";
import MenuText from "../../../../menu/menu-grid/ui/MenuText";
import MenuTitle from "../../../../menu/menu-title/ui/MenuTitle";
import { StyleComponent } from "../../../../../../../../../components/local/data/style.component";

export default function SubmenuTitle({ title, items }: SubmenuTitleProps) {
  return <div className={StyleComponent.menu.submenu.title.root}>
    {title && <MenuText {...title} />}
    {items && <div className={StyleComponent.menu.submenu.title.groups}>{items.map((item, i) => <MenuTitle key={i} {...item} />)}</div>}
  </div>;
}
