import type { CardMediaProps } from "../../../../../../molecules/card/media/data/types";
import type { SectionCarouselButtonsProps } from "../../../buttons/data/types";

export type CarouselCardMediaProps = {
  sectionCarouselButtonsProps?: Omit<SectionCarouselButtonsProps, "children">;
  items: CardMediaProps[];
};
