<script lang="ts">
  import GridCardMedia from "../../../../../../../project/ui/organisms/section/grid/card/media/ui/GridCardMedia.svelte";

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

<GridCardMedia
  sectionGridProps={{
    titleProps: { text: "GridCardMedia" },
    descriptionProps: { text: "Cartões apresentados com um componente reutilizável." },
  }}
  {items}
/>
