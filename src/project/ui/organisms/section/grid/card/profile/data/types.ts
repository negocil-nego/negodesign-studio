import type { CardProfileProps } from "../../../../../../molecules/card/profile/data/types";
import type { SectionGridProps } from "../../../data/types";

export type GridCardProfileProps = {
  sectionGridProps?: Omit<SectionGridProps, "children">;
  items: CardProfileProps[];
};
