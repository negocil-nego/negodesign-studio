<script lang="ts">
  import CarouselCardProfile from "../../../../../../../project/ui/organisms/section/carousel/card/profile/ui/CarouselCardProfile.svelte";

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

<CarouselCardProfile
  sectionCarouselButtonsProps={{
    titleProps: { text: "CarouselCardProfile" },
    descriptionProps: { text: "Cartões apresentados com um componente reutilizável." },
  }}
  {items}
/>
