import type { SubmenuGridTitleProps } from "../data/types";
import MenuTitle from "../../../../menu/menu-title/ui/MenuTitle";
import SubmenuTitle from "../../../simple/title/ui/SubmenuTitle";
import { StyleComponent } from "../../../../../../../../../components/local/data/style.component";

export default function SubmenuGridTitle({ title, items }: SubmenuGridTitleProps) {
  return <div className={StyleComponent.menu.submenu.gridTitle.root}>
    {title && <aside className={StyleComponent.menu.submenu.gridTitle.intro}><MenuTitle {...title} variant="featured" /></aside>}
    {!!items?.length && <div className={StyleComponent.menu.submenu.gridTitle.groups}>{items.map((item, i) => <div key={i} className={StyleComponent.menu.submenu.gridTitle.group}><SubmenuTitle {...item} /></div>)}</div>}
  </div>;
}
