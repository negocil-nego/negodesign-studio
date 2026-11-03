import type { CardProductProps } from "../../../../../../molecules/card/product/data/types";
import type { SectionGridCarouselProps } from "../../../data/types";

export type GridCarouselCardProductProps = {
  sectionGridCarouselProps?: Omit<SectionGridCarouselProps, "children">;
  items: CardProductProps[];
};
