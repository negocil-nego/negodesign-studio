import type { Snippet } from "svelte";
import type { TextProps } from "../../../../atoms/text/data/types";
export type SectionGridProps = {
  titleProps?: TextProps;
  descriptionProps?: TextProps;
  className?: string;
  children: Snippet;
};
