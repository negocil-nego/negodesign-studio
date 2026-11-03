import CarouselCardProduct from "../../../../../../../project/ui/organisms/section/carousel/card/product/ui/CarouselCardProduct";
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

export default function CarouselCardProductExample() {
  return (
    <CarouselCardProduct
      sectionCarouselButtonsProps={{
        titleProps: { text: "CarouselCardProduct" },
        descriptionProps: { text: "Cartões apresentados com um componente reutilizável." },
      }}
      items={items}
    />
  );
}
