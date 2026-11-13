<script setup lang="ts">
import { StyleComponent } from "../../../../../data/style.component";
import ProductGallery from "../../shared/product-gallery/ui/ProductGallery.vue";
import ProductInfo from "../../shared/product-info/ui/ProductInfo.vue";
import ProductRating from "../../shared/product-rating/ui/ProductRating.vue";
import ProductSimiliar from "../../shared/product-similiar/ui/ProductSimiliar.vue";
import ProductTabs from "../../shared/product-tabs/ui/ProductTabs.vue";
import type { ProductDetailsLayoutProps } from "../../data/types";

defineProps<ProductDetailsLayoutProps>();

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

  <div :class="S.sidebarLayout">
    <div :class="S.mainCol">
      <section v-if="productTabsProps" :class="S.sectionCard">
        <ProductTabs v-bind="productTabsProps" />
      </section>
      <section v-if="productRatingProps" :class="S.sectionCard">
        <ProductRating v-bind="productRatingProps" />
      </section>
    </div>
    <div :class="S.sideCol">
      <ProductSimiliar v-if="productSimiliarProps" v-bind="productSimiliarProps" variant="sidebar" />
    </div>
  </div>
</template>
