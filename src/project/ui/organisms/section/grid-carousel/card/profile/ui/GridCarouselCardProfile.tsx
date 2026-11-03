import type { GridCarouselCardProfileProps } from "../data/types";
import CardProfile from "../../../../../../molecules/card/profile/ui/CardProfile";
import SectionGridCarousel from "../../../ui/SectionGridCarousel";

export default function GridCarouselCardProfile({ items, sectionGridCarouselProps }: GridCarouselCardProfileProps) {
  return (
    <SectionGridCarousel {...sectionGridCarouselProps}>
      {items.map((item, index) => (
        <CardProfile key={index} {...item} />
      ))}
    </SectionGridCarousel>
  );
}
