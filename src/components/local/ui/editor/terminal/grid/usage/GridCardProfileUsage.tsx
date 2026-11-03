import GridCardProfile from "../../../../../../../project/ui/organisms/section/grid/card/profile/ui/GridCardProfile";
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

export default function GridCardProfileExample() {
  return (
    <GridCardProfile
      sectionGridProps={{
        titleProps: { text: "GridCardProfile" },
        descriptionProps: { text: "Cartões apresentados com um componente reutilizável." },
      }}
      items={items}
    />
  );
}
