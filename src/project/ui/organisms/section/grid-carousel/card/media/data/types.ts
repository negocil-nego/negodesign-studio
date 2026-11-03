import type { CardMediaProps } from "../../../../../../molecules/card/media/data/types";
import type { SectionGridCarouselProps } from "../../../data/types";

export type GridCarouselCardMediaProps = {
  sectionGridCarouselProps?: Omit<SectionGridCarouselProps, "children">;
  items: CardMediaProps[];
};
