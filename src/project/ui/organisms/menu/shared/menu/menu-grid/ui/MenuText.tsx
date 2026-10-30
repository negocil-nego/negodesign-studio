import type { MenuTextProps } from "../../menu-text/data/types";
import { StyleComponent } from "../../../../../../../../components/local/data/style.component";
import Icon from "../../../../../../atoms/icon/ui/Icon";
import Text from "../../../../../../atoms/text/ui/Text";

export default function MenuText({ icon, textProps }: MenuTextProps) {
  return <div className={StyleComponent.menu.text.root}>
    {icon && <Icon name={icon.name} isLoading={icon.isLoading} className={`${StyleComponent.menu.text.icon} ${icon.className ?? ""}`} />}
    {textProps && <div className={StyleComponent.menu.text.label}><Text {...textProps} /></div>}
  </div>;
}
