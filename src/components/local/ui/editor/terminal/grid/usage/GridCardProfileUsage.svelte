<script lang="ts">
  import GridCardProfile from "../../../../../../../project/ui/organisms/section/grid/card/profile/ui/GridCardProfile.svelte";

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

<GridCardProfile
  sectionGridProps={{
    titleProps: { text: "GridCardProfile" },
    descriptionProps: { text: "Cartões apresentados com um componente reutilizável." },
  }}
  {items}
/>
