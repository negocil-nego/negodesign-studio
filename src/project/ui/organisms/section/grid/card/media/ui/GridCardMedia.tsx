import type { GridCardMediaProps } from "../data/types";
import CardMedia from "../../../../../../molecules/card/media/ui/CardMedia";
import SectionGrid from "../../../ui/SectionGrid";

export default function GridCardMedia({ items, sectionGridProps }: GridCardMediaProps) {
  return (
    <SectionGrid {...sectionGridProps}>
      {items.map((item, index) => (
        <CardMedia key={index} {...item} />
      ))}
    </SectionGrid>
  );
}
