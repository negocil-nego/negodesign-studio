<script setup lang="ts">
import SectionGrid from "../../../../../../../project/ui/organisms/section/grid/ui/SectionGrid.vue";
import CardMedia from "../../../../../../../project/ui/molecules/card/media/ui/CardMedia.vue";
import CardProduct from "../../../../../../../project/ui/molecules/card/product/ui/CardProduct.vue";
</script>

<template>
  <SectionGrid
    :title-props="{ text: 'SectionGrid' }"
    :description-props="{ text: 'Organize cartões numa apresentação responsiva.' }"
  >
    <CardMedia :title-props="{ title: 'Design system' }" :image-props="{ src: '/img/ux-01.jpg', alt: 'Equipa a trabalhar' }" />
    <CardProduct :title-props="{ title: 'Componentes reutilizáveis' }" :image-props="{ src: '/img/programmer-01.jpg', alt: 'Código num monitor' }" />
  </SectionGrid>
</template>
