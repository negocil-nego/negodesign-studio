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

export const HERO_GRID_COLS: Record<number, string> = {
  1: "md:col-span-1",
  2: "md:col-span-2",
  3: "md:col-span-3",
  4: "md:col-span-4",
  5: "md:col-span-5",
  6: "md:col-span-6",
  7: "md:col-span-7",
  8: "md:col-span-8",
  9: "md:col-span-9",
  10: "md:col-span-10",
  11: "md:col-span-11",
  12: "md:col-span-12",
};

export const HERO_GRID_ROWS: Record<number, string> = {
  1: "md:row-span-1",
  2: "md:row-span-2",
  3: "md:row-span-3",
  4: "md:row-span-4",
  5: "md:row-span-5",
  6: "md:row-span-6",
  7: "md:row-span-7",
};

export const HERO_GRID_POS: Record<string, string> = {
  start: "justify-start",
  center: "justify-center",
  end: "justify-end",
  between: "justify-between",
};
