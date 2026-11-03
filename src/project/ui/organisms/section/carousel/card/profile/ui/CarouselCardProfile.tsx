import type { CarouselCardProfileProps } from "../data/types";
import CardProfile from "../../../../../../molecules/card/profile/ui/CardProfile";
import SectionCarouselButtons from "../../../buttons/ui/SectionCarouselButtons";

export default function CarouselCardProfile({ items, sectionCarouselButtonsProps }: CarouselCardProfileProps) {
  return (
    <SectionCarouselButtons {...sectionCarouselButtonsProps}>
      {items.map((item, index) => (
        <CardProfile key={index} {...item} />
      ))}
    </SectionCarouselButtons>
  );
}
