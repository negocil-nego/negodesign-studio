<script setup lang="ts">
import type { GridCarouselCardProductProps } from "../data/types";
import CardProduct from "../../../../../../molecules/card/product/ui/CardProduct.vue";
import SectionGridCarousel from "../../../ui/SectionGridCarousel.vue";

const props = defineProps<GridCarouselCardProductProps>();
</script>

<template>
  <SectionGridCarousel v-bind="props.sectionGridCarouselProps">
    <CardProduct
      v-for="(item, index) in props.items"
      :key="index"
      v-bind="item"
    />
  </SectionGridCarousel>
</template>
