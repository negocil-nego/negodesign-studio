import type { GridCardProductProps } from "../data/types";
import CardProduct from "../../../../../../molecules/card/product/ui/CardProduct";
import SectionGrid from "../../../ui/SectionGrid";

export default function GridCardProduct({ items, sectionGridProps }: GridCardProductProps) {
  return (
    <SectionGrid {...sectionGridProps}>
      {items.map((item, index) => (
        <CardProduct key={index} {...item} />
      ))}
    </SectionGrid>
  );
}
