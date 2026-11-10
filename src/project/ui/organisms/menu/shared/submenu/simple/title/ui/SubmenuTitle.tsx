import type { SubmenuTitleProps } from "../data/types";
import MenuText from "../../../../menu/menu-grid/ui/MenuText";
import MenuTitle from "../../../../menu/menu-title/ui/MenuTitle";
import { StyleComponent } from "../../../../../../../../data/style.component";

export default function SubmenuTitle({ title, items }: SubmenuTitleProps) {
  const s = StyleComponent.menu.submenu.title;
  return (
    <div className={s.root}>
      {title && <MenuText {...title} />}
      {items && (
        <div className={s.groups}>
          {items.map((item, i) => (
            <MenuTitle key={i} {...item} />
          ))}
        </div>
      )}
    </div>
  );
}
