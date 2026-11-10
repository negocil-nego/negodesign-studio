import type { SubmenuGridTitleProps } from "../data/types";
import MenuTitle from "../../../../menu/menu-title/ui/MenuTitle";
import SubmenuTitle from "../../../simple/title/ui/SubmenuTitle";
import { StyleComponent } from "../../../../../../../../data/style.component";

export default function SubmenuGridTitle({ title, items }: SubmenuGridTitleProps) {
  const s = StyleComponent.menu.submenu.gridTitle;
  return (
    <div className={s.root}>
      {title && (
        <aside className={s.intro}>
          <MenuTitle {...title} variant="featured" />
        </aside>
      )}
      {!!items?.length && (
        <div className={s.groups}>
          {items.map((item, i) => (
            <div key={i} className={s.group}>
              <SubmenuTitle {...item} />
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
