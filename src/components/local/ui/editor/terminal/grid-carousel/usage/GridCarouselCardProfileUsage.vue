<script setup lang="ts">
import GridCarouselCardProfile from "../../../../../../../project/ui/organisms/section/grid-carousel/card/profile/ui/GridCarouselCardProfile.vue";
import type { CardProfileProps } from "../../../../../../../project/ui/molecules/card/profile/data/types";

const items: CardProfileProps[] = [
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
  titleProps: { text: "GridCarouselCardProfile" },
  descriptionProps: { text: "Cartões apresentados com um componente reutilizável." },
  rows: 2,
  columns: 2,
};
</script>

<template>
  <GridCarouselCardProfile :section-grid-carousel-props="sectionProps" :items="items" />
</template>
