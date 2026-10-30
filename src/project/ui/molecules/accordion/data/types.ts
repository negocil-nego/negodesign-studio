import type { Snippet } from "svelte";

export type AccordionProps = {
  title: string;
  count?: string;
  open?: boolean;
  children: Snippet;
};
