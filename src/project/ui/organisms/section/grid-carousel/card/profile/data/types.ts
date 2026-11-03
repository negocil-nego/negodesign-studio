import type { CardProfileProps } from "../../../../../../molecules/card/profile/data/types";
import type { SectionGridCarouselProps } from "../../../data/types";

export type GridCarouselCardProfileProps = {
  sectionGridCarouselProps?: Omit<SectionGridCarouselProps, "children">;
  items: CardProfileProps[];
};
