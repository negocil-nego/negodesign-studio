import type { CarouselCardProductProps } from "../data/types";
import CardProduct from "../../../../../../molecules/card/product/ui/CardProduct";
import SectionCarouselButtons from "../../../buttons/ui/SectionCarouselButtons";

export default function CarouselCardProduct({ items, sectionCarouselButtonsProps }: CarouselCardProductProps) {
  return (
    <SectionCarouselButtons {...sectionCarouselButtonsProps}>
      {items.map((item, index) => (
        <CardProduct key={index} {...item} />
      ))}
    </SectionCarouselButtons>
  );
}
