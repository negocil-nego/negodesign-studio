<script setup lang="ts">
import { ref } from "vue";
import ProductInfoHeader from "../sections/ProductInfoHeader.vue";
import ProductInfoOptions from "../sections/ProductInfoOptions.vue";
import ProductInfoActions from "../sections/ProductInfoActions.vue";
import type { ProductInfoProps } from "../data/types";

const props = withDefaults(defineProps<ProductInfoProps>(), {
  featuresTitle: "Features:",
  features: () => [],
  colorsLabel: "Colors",
  colors: () => [],
  sizesLabel: "Size",
  sizes: () => [],
  purchaseOptions: () => [],
  showQuantity: false,
  initialQuantity: 1,
  trustBadges: () => [],
  metaItems: () => [],
  accordionItems: () => [],
  jumpLinks: () => [],
  className: "",
});

const quantity = ref(props.initialQuantity);
const selectedColor = ref(props.colors[0]?.value ?? "");
const selectedSize = ref(props.sizes[0]?.value ?? "");
</script>

<template>
  <div :class="`space-y-5 ${className}`">
    <ProductInfoHeader
      :category="category" :badge="badge" :stock-badge="stockBadge" :title-props="titleProps"
      :subtitle="subtitle" :rating-props="ratingProps" :review-count-text="reviewCountText"
      :price-props="priceProps" :price-unit="priceUnit" :old-price="oldPrice"
      :discount-text="discountText" :description-props="descriptionProps"
      :features-title="featuresTitle" :features="features"
    />
    <ProductInfoOptions
      :colors-label="colorsLabel" :colors="colors" :sizes-label="sizesLabel" :sizes="sizes"
      :show-quantity="showQuantity" :quantity="quantity" :selected-color="selectedColor"
      :selected-size="selectedSize"
      @update:quantity="quantity = $event"
      @update:selectedColor="selectedColor = $event"
      @update:selectedSize="selectedSize = $event"
    />
    <ProductInfoActions
      :button-props="buttonProps" :secondary-button-props="secondaryButtonProps"
      :cart-button-props="cartButtonProps" :favorite-button-props="favoriteButtonProps"
      :tags-props="tagsProps" :trust-badges="trustBadges"
    />
  </div>
</template>
