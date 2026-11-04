import type { MenuProps } from "../../../../organisms/menu/simple/data/types";
import type { SectionCardGridProps } from "../../../../organisms/section/hero/grid/data/types";

export type PageHeroGridCardProps = {
  id: string;
  colSpan?: 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12;
  rowSpan?: 1 | 2 | 3 | 4 | 5 | 6 | 7;
  className?: string;
  backgroundColor?: string;
  imageUrl?: string;
  imageAlt?: string;
  imageMode?: "cover" | "contain";
  showOverlay?: boolean;
  imageClassName?: string;
  sectionCardProps?: SectionCardGridProps;
};

export type PageHeroGridProps = {
  menuProps?: MenuProps;
  backgroundColor?: string;
  cards?: PageHeroGridCardProps[];
};
