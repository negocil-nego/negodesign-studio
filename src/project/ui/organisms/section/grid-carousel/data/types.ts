import type { Snippet } from "svelte";
import type { TextProps } from "../../../../atoms/text/data/types";
export type SectionGridCarouselProps = {
  titleProps?: TextProps;
  descriptionProps?: TextProps;
  className?: string;
  rows?: number;
  columns?: number;
  children: Snippet;
};
