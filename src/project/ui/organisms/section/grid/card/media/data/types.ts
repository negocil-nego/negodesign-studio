import type { CardMediaProps } from "../../../../../../molecules/card/media/data/types";
import type { SectionGridProps } from "../../../data/types";

export type GridCardMediaProps = {
  sectionGridProps?: Omit<SectionGridProps, "children">;
  items: CardMediaProps[];
};
