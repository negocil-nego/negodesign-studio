<script setup lang="ts">
import type { CarouselCardProductProps } from "../data/types";
import CardProduct from "../../../../../../molecules/card/product/ui/CardProduct.vue";
import SectionCarouselButtons from "../../../buttons/ui/SectionCarouselButtons.vue";

const props = defineProps<CarouselCardProductProps>();
</script>

<template>
  <SectionCarouselButtons v-bind="props.sectionCarouselButtonsProps">
    <CardProduct
      v-for="(item, index) in props.items"
      :key="index"
      v-bind="item"
    />
  </SectionCarouselButtons>
</template>
