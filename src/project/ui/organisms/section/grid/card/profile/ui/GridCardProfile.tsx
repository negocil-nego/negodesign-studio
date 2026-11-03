import type { GridCardProfileProps } from "../data/types";
import CardProfile from "../../../../../../molecules/card/profile/ui/CardProfile";
import SectionGrid from "../../../ui/SectionGrid";

export default function GridCardProfile({ items, sectionGridProps }: GridCardProfileProps) {
  return (
    <SectionGrid {...sectionGridProps}>
      {items.map((item, index) => (
        <CardProfile key={index} {...item} />
      ))}
    </SectionGrid>
  );
}
