<script lang="ts">
  import CarouselCardProduct from "../../../../../../../project/ui/organisms/section/carousel/card/product/ui/CarouselCardProduct.svelte";

  const items = [
  {
    titleProps: { title: "Design system" },
    imageProps: { src: "/img/ux-01.jpg", alt: "Equipa a trabalhar num design system" },
  },
  {
    titleProps: { title: "Componentes reutilizáveis" },
    imageProps: { src: "/img/programmer-01.jpg", alt: "Componentes de software" },
  },
];
</script>

<CarouselCardProduct
  sectionCarouselButtonsProps={{
    titleProps: { text: "CarouselCardProduct" },
    descriptionProps: { text: "Cartões apresentados com um componente reutilizável." },
  }}
  {items}
/>
