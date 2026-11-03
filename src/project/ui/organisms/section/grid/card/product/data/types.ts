import type { CardProductProps } from "../../../../../../molecules/card/product/data/types";
import type { SectionGridProps } from "../../../data/types";

export type GridCardProductProps = {
  sectionGridProps?: Omit<SectionGridProps, "children">;
  items: CardProductProps[];
};
