<script setup lang="ts">
import GridCarouselCardProduct from "../../../../../../../project/ui/organisms/section/grid-carousel/card/product/ui/GridCarouselCardProduct.vue";
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
const sectionProps = {
  titleProps: { text: "GridCarouselCardProduct" },
  descriptionProps: { text: "Cartões apresentados com um componente reutilizável." },
  rows: 2,
  columns: 2,
};
</script>

<template>
  <GridCarouselCardProduct :section-grid-carousel-props="sectionProps" :items="items" />
</template>
