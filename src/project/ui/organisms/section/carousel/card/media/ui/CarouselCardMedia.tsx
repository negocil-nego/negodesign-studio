import type { CarouselCardMediaProps } from "../data/types";
import CardMedia from "../../../../../../molecules/card/media/ui/CardMedia";
import SectionCarouselButtons from "../../../buttons/ui/SectionCarouselButtons";

export default function CarouselCardMedia({ items, sectionCarouselButtonsProps }: CarouselCardMediaProps) {
  return (
    <SectionCarouselButtons {...sectionCarouselButtonsProps}>
      {items.map((item, index) => (
        <CardMedia key={index} {...item} />
      ))}
    </SectionCarouselButtons>
  );
}
