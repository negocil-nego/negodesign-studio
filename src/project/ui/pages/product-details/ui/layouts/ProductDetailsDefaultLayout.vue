<script setup lang="ts">
import GridCardMedia from "../../../../organisms/section/grid/card/media/ui/GridCardMedia.vue";
import { StyleComponent } from "../../../../../data/style.component";
import ProductGallery from "../../shared/product-gallery/ui/ProductGallery.vue";
import ProductInfo from "../../shared/product-info/ui/ProductInfo.vue";
import ProductRating from "../../shared/product-rating/ui/ProductRating.vue";
import ProductSimiliar from "../../shared/product-similiar/ui/ProductSimiliar.vue";
import ProductTabs from "../../shared/product-tabs/ui/ProductTabs.vue";
import type { ProductDetailsDefaultLayoutProps } from "../../data/types";

defineProps<ProductDetailsDefaultLayoutProps>();

const S = StyleComponent.pages.productDetails;
</script>

<template>
  <section :class="S.heroGrid">
    <div :class="S.galleryCol">
      <ProductGallery v-if="productGalleryProps" v-bind="productGalleryProps" />
    </div>
    <div :class="S.infoCol">
      <ProductInfo v-if="productInfoProps" v-bind="productInfoProps" />
    </div>
  </section>

  <section v-if="featureCardsProps" :class="S.sectionCard">
    <GridCardMedia v-bind="featureCardsProps" />
  </section>

  <section v-if="productTabsProps && variant !== 'minimal'" :class="S.sectionCard">
    <ProductTabs v-bind="productTabsProps" />
  </section>

  <section v-if="productRatingProps && variant !== 'minimal'" :class="S.sectionCard">
    <ProductRating v-bind="productRatingProps" />
  </section>

  <section v-if="productSimiliarProps" :class="S.sectionCard">
    <ProductSimiliar v-bind="productSimiliarProps" />
  </section>
</template>
