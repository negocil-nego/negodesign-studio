import type { CardProductProps } from "../../../../../../molecules/card/product/data/types";
import type { SectionCarouselButtonsProps } from "../../../buttons/data/types";

export type CarouselCardProductProps = {
  sectionCarouselButtonsProps?: Omit<SectionCarouselButtonsProps, "children">;
  items: CardProductProps[];
};
