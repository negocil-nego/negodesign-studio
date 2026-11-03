<script lang="ts">
  import GridCarouselCardProfile from "../../../../../../../project/ui/organisms/section/grid-carousel/card/profile/ui/GridCarouselCardProfile.svelte";

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

<GridCarouselCardProfile
  sectionGridCarouselProps={{
    titleProps: { text: "GridCarouselCardProfile" },
    descriptionProps: { text: "Cartões apresentados com um componente reutilizável." },
    rows: 2,
    columns: 2,
  }}
  {items}
/>
