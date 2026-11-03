<script setup lang="ts">
import GridCarouselCardMedia from "../../../../../../../project/ui/organisms/section/grid-carousel/card/media/ui/GridCarouselCardMedia.vue";
import type { CardMediaProps } from "../../../../../../../project/ui/molecules/card/media/data/types";

const items: CardMediaProps[] = [
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
  titleProps: { text: "GridCarouselCardMedia" },
  descriptionProps: { text: "Cartões apresentados com um componente reutilizável." },
  rows: 2,
  columns: 2,
};
</script>

<template>
  <GridCarouselCardMedia :section-grid-carousel-props="sectionProps" :items="items" />
</template>
