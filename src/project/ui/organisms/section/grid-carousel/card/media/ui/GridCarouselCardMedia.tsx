import type { GridCarouselCardMediaProps } from "../data/types";
import CardMedia from "../../../../../../molecules/card/media/ui/CardMedia";
import SectionGridCarousel from "../../../ui/SectionGridCarousel";

export default function GridCarouselCardMedia({ items, sectionGridCarouselProps }: GridCarouselCardMediaProps) {
  return (
    <SectionGridCarousel {...sectionGridCarouselProps}>
      {items.map((item, index) => (
        <CardMedia key={index} {...item} />
      ))}
    </SectionGridCarousel>
  );
}
