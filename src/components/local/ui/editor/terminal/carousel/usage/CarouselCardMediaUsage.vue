<script setup lang="ts">
import CarouselCardMedia from "../../../../../../../project/ui/organisms/section/carousel/card/media/ui/CarouselCardMedia.vue";
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
</script>

<template>
  <CarouselCardMedia
    :items="items"
    :section-carousel-buttons-props="{
      titleProps: { text: 'Projectos recentes' },
      descriptionProps: { text: 'Uma selecção de trabalhos da equipa.' },
    }"
  />
</template>
