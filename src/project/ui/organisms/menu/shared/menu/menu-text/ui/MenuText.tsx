import type { MenuTextProps } from "../data/types";
import Icon from "../../../../../../atoms/icon/ui/Icon";
import Text from "../../../../../../atoms/text/ui/Text";

export default function MenuText({ icon, textProps }: MenuTextProps) {
  return <>{icon && <Icon {...icon} />}{textProps && <Text {...textProps} />}</>;
}
