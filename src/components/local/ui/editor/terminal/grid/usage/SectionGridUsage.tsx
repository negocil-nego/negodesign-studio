import SectionGrid from "../../../../../../../project/ui/organisms/section/grid/ui/SectionGrid";
import CardMedia from "../../../../../../../project/ui/molecules/card/media/ui/CardMedia";
import CardProduct from "../../../../../../../project/ui/molecules/card/product/ui/CardProduct";

export default function SectionGridExample() {
  return (
    <SectionGrid
      titleProps={{ text: "SectionGrid" }}
      descriptionProps={{ text: "Organize cartões numa apresentação responsiva." }}
    >
      <CardMedia titleProps={{ title: "Design system" }} imageProps={{ src: "/img/ux-01.jpg", alt: "Equipa a trabalhar" }} />
      <CardProduct titleProps={{ title: "Componentes reutilizáveis" }} imageProps={{ src: "/img/programmer-01.jpg", alt: "Código num monitor" }} />
    </SectionGrid>
  );
}
