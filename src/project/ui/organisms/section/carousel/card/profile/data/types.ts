import type { CardProfileProps } from "../../../../../../molecules/card/profile/data/types";
import type { SectionCarouselButtonsProps } from "../../../buttons/data/types";

export type CarouselCardProfileProps = {
  sectionCarouselButtonsProps?: Omit<SectionCarouselButtonsProps, "children">;
  items: CardProfileProps[];
};
