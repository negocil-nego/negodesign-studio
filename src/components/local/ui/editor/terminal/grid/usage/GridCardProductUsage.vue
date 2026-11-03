<script setup lang="ts">
import GridCardProduct from "../../../../../../../project/ui/organisms/section/grid/card/product/ui/GridCardProduct.vue";
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
  titleProps: { text: "GridCardProduct" },
  descriptionProps: { text: "Cartões apresentados com um componente reutilizável." },
};
</script>

<template>
  <GridCardProduct :section-grid-props="sectionProps" :items="items" />
</template>
