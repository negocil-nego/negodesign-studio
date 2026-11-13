<script setup lang="ts">
import CardDescription from "../../../../../molecules/card/shared/card-description/ui/CardDescription.vue";
import CardPrice from "../../../../../molecules/card/shared/card-price/ui/CardPrice.vue";
import CardRating from "../../../../../molecules/card/shared/card-rating/ui/CardRating.vue";
import CardTitle from "../../../../../molecules/card/shared/card-title/ui/CardTitle.vue";
import type { ProductInfoHeaderProps } from "../data/types";

withDefaults(defineProps<ProductInfoHeaderProps>(), {
  featuresTitle: "Features:",
  features: () => [],
});
</script>

<template>
  <div class="space-y-2">
    <div v-if="category || badge || stockBadge" class="flex flex-wrap items-center justify-between gap-2">
      <div class="flex flex-wrap items-center gap-2">
        <span v-if="category" class="text-xs font-medium uppercase tracking-wider text-blue-400">{{ category }}</span>
        <span v-if="badge" class="rounded-full border border-emerald-500/40 bg-emerald-500/10 px-2.5 py-0.5 text-[11px] font-semibold text-emerald-300">
          {{ badge }}
        </span>
      </div>
      <span v-if="stockBadge" class="rounded-md border border-blue-500/40 bg-blue-500/10 px-2.5 py-0.5 text-[11px] font-semibold text-blue-300">
        {{ stockBadge }}
      </span>
    </div>

    <CardTitle v-if="titleProps" v-bind="titleProps" />

    <div class="flex flex-wrap items-center gap-3 text-xs text-zinc-400">
      <span v-if="subtitle">{{ subtitle }}</span>
      <div v-if="ratingProps" class="flex items-center gap-2">
        <CardRating v-bind="ratingProps" />
        <span class="font-semibold text-zinc-200">{{ ratingProps.rating }}</span>
        <span v-if="reviewCountText">({{ reviewCountText }})</span>
      </div>
    </div>
  </div>

  <div v-if="priceProps || oldPrice || discountText" class="space-y-1">
    <div class="flex flex-wrap items-baseline gap-2.5">
      <CardPrice v-if="priceProps" v-bind="priceProps" />
      <span v-if="priceUnit" class="text-xs text-zinc-400">{{ priceUnit }}</span>
      <span v-if="oldPrice" class="text-sm text-zinc-500 line-through">{{ oldPrice }}</span>
    </div>
    <p v-if="discountText" class="text-xs font-medium text-emerald-400">{{ discountText }}</p>
  </div>

  <CardDescription v-if="descriptionProps" v-bind="descriptionProps" />

  <div v-if="features.length > 0" class="space-y-2">
    <h4 v-if="featuresTitle" class="text-xs font-semibold text-zinc-200">{{ featuresTitle }}</h4>
    <ul class="space-y-1.5 text-xs text-zinc-300">
      <li v-for="feat in features" :key="feat" class="flex items-center gap-2">
        <span class="text-blue-400">✓</span>
        <span>{{ feat }}</span>
      </li>
    </ul>
  </div>
</template>
