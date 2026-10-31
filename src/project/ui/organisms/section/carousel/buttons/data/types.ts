import type { Snippet } from "svelte";
import type { TextProps } from "../../../../../atoms/text/data/types";

export type SectionCarouselButtonsProps = {
  titleProps?: TextProps;
  descriptionProps?: TextProps;
  className?: string;
  children: Snippet;
};
