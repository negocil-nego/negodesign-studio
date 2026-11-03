import GridCardProduct from "../../../../../../../project/ui/organisms/section/grid/card/product/ui/GridCardProduct";
import type { CardProductProps } from "../../../../../../../project/ui/molecules/card/product/data/types";

const items: CardProductProps[] = [
  {
    titleProps: { title: "Design system" },
    imageProps: { src: "/img/ux-01.jpg", alt: "Equipa a trabalhar num design system" },
  },
  {
    titleProps: { title: "Componentes reutilizáveis" },
    imageProps: { src: "/img/programmer-01.jpg", alt: "Componentes de software" },
  },
];

export default function GridCardProductExample() {
  return (
    <GridCardProduct
      sectionGridProps={{
        titleProps: { text: "GridCardProduct" },
        descriptionProps: { text: "Cartões apresentados com um componente reutilizável." },
      }}
      items={items}
    />
  );
}
