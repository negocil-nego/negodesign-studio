<script setup lang="ts">
import SectionGridCarousel from "../../../../../../../project/ui/organisms/section/grid-carousel/ui/SectionGridCarousel.vue";
import CardMedia from "../../../../../../../project/ui/molecules/card/media/ui/CardMedia.vue";
import CardProduct from "../../../../../../../project/ui/molecules/card/product/ui/CardProduct.vue";
</script>

<template>
  <SectionGridCarousel
    :title-props="{ text: 'SectionGridCarousel' }"
    :description-props="{ text: 'Organize cartões numa apresentação responsiva.' }"
    :rows="2"
    :columns="2"
  >
    <CardMedia :title-props="{ title: 'Design system' }" :image-props="{ src: '/img/ux-01.jpg', alt: 'Equipa a trabalhar' }" />
    <CardProduct :title-props="{ title: 'Componentes reutilizáveis' }" :image-props="{ src: '/img/programmer-01.jpg', alt: 'Código num monitor' }" />
  </SectionGridCarousel>
</template>
