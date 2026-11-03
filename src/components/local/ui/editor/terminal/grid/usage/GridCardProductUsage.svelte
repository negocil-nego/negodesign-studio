<script lang="ts">
  import GridCardProduct from "../../../../../../../project/ui/organisms/section/grid/card/product/ui/GridCardProduct.svelte";

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

<GridCardProduct
  sectionGridProps={{
    titleProps: { text: "GridCardProduct" },
    descriptionProps: { text: "Cartões apresentados com um componente reutilizável." },
  }}
  {items}
/>
