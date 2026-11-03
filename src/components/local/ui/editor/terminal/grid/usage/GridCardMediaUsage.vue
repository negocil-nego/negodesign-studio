<script setup lang="ts">
import GridCardMedia from "../../../../../../../project/ui/organisms/section/grid/card/media/ui/GridCardMedia.vue";
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
  titleProps: { text: "GridCardMedia" },
  descriptionProps: { text: "Cartões apresentados com um componente reutilizável." },
};
</script>

<template>
  <GridCardMedia :section-grid-props="sectionProps" :items="items" />
</template>
