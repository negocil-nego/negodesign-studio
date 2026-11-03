import type { GridCarouselCardProductProps } from "../data/types";
import CardProduct from "../../../../../../molecules/card/product/ui/CardProduct";
import SectionGridCarousel from "../../../ui/SectionGridCarousel";

export default function GridCarouselCardProduct({ items, sectionGridCarouselProps }: GridCarouselCardProductProps) {
  return (
    <SectionGridCarousel {...sectionGridCarouselProps}>
      {items.map((item, index) => (
        <CardProduct key={index} {...item} />
      ))}
    </SectionGridCarousel>
  );
}
