import CarouselCardProfile from "../../../../../../../project/ui/organisms/section/carousel/card/profile/ui/CarouselCardProfile";
import type { CardProfileProps } from "../../../../../../../project/ui/molecules/card/profile/data/types";

const items: CardProfileProps[] = [
  {
    titleProps: { title: "Design system" },
    imageProps: { src: "/img/ux-01.jpg", alt: "Equipa a trabalhar num design system" },
  },
  {
    titleProps: { title: "Componentes reutilizáveis" },
    imageProps: { src: "/img/programmer-01.jpg", alt: "Componentes de software" },
  },
];

export default function CarouselCardProfileExample() {
  return (
    <CarouselCardProfile
      sectionCarouselButtonsProps={{
        titleProps: { text: "CarouselCardProfile" },
        descriptionProps: { text: "Cartões apresentados com um componente reutilizável." },
      }}
      items={items}
    />
  );
}
