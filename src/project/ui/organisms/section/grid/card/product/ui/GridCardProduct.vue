<script setup lang="ts">
import type { GridCardProductProps } from "../data/types";
import CardProduct from "../../../../../../molecules/card/product/ui/CardProduct.vue";
import SectionGrid from "../../../ui/SectionGrid.vue";

const props = defineProps<GridCardProductProps>();
</script>

<template>
  <SectionGrid v-bind="props.sectionGridProps">
    <CardProduct
      v-for="(item, index) in props.items"
      :key="index"
      v-bind="item"
    />
  </SectionGrid>
</template>
