<script setup lang="ts">
import { ref } from "vue";
import Icon from "../../../../../atoms/icon/ui/Icon.vue";
import CardButton from "../../../../../molecules/card/shared/card-button/ui/CardButton.vue";
import CardCartButton from "../../../../../molecules/card/shared/card-cart-button/ui/CardCartButton.vue";
import CardDescription from "../../../../../molecules/card/shared/card-description/ui/CardDescription.vue";
import CardFavoriteButton from "../../../../../molecules/card/shared/card-favorite-button/ui/CardFavoriteButton.vue";
import CardPrice from "../../../../../molecules/card/shared/card-price/ui/CardPrice.vue";
import CardRating from "../../../../../molecules/card/shared/card-rating/ui/CardRating.vue";
import CardTags from "../../../../../molecules/card/shared/card-tags/ui/CardTags.vue";
import CardTitle from "../../../../../molecules/card/shared/card-title/ui/CardTitle.vue";
import ProductTabs from "../../product-tabs/ui/ProductTabs.vue";
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
    <div class="space-y-2">
      <div v-if="category || badge || stockBadge" class="flex flex-wrap items-center justify-between gap-2">
        <div class="flex flex-wrap items-center gap-2">
          <span v-if="category" class="text-xs font-medium uppercase tracking-wider text-blue-400">
            {{ category }}
          </span>
          <span
            v-if="badge"
            class="rounded-full border border-emerald-500/40 bg-emerald-500/10 px-2.5 py-0.5 text-[11px] font-semibold text-emerald-300"
          >
            {{ badge }}
          </span>
        </div>
        <span
          v-if="stockBadge"
          class="rounded-md border border-blue-500/40 bg-blue-500/10 px-2.5 py-0.5 text-[11px] font-semibold text-blue-300"
        >
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

    <div v-if="colors.length > 0" class="space-y-2">
      <span class="block text-xs font-semibold text-zinc-300">{{ colorsLabel }}</span>
      <div class="flex flex-wrap items-center gap-2.5">
        <button
          v-for="c in colors"
          :key="c.value"
          type="button"
          :style="{ backgroundColor: c.hex }"
          class="flex size-7 items-center justify-center rounded-md border border-blender-border text-[10px] font-bold text-white"
          @click="selectedColor = c.value"
        >
          {{ selectedColor === c.value ? "✓" : "" }}
        </button>
      </div>
    </div>

    <div v-if="sizes.length > 0" class="space-y-2">
      <span class="block text-xs font-semibold text-zinc-300">{{ sizesLabel }}</span>
      <div class="flex flex-wrap items-center gap-2">
        <button
          v-for="s in sizes"
          :key="s.value"
          type="button"
          :class="`rounded-lg border px-3 py-1.5 text-xs font-medium ${
            selectedSize === s.value
              ? 'border-blue-500 bg-blue-600 text-white'
              : 'border-blender-border bg-blender-surface text-zinc-300'
          }`"
          @click="selectedSize = s.value"
        >
          {{ s.priceLabel ? `${s.label} = ${s.priceLabel}` : s.label }}
        </button>
      </div>
    </div>

    <div v-if="showQuantity" class="flex items-center gap-4">
      <div class="inline-flex items-center rounded-lg border border-blender-border bg-blender-surface">
        <button type="button" class="px-3 py-1.5 text-xs font-bold text-zinc-300" @click="quantity = Math.max(1, quantity - 1)">−</button>
        <span class="min-w-8 text-center text-xs font-semibold text-zinc-100">{{ quantity }}</span>
        <button type="button" class="px-3 py-1.5 text-xs font-bold text-zinc-300" @click="quantity += 1">+</button>
      </div>
    </div>

    <div class="flex flex-wrap items-center gap-2.5 pt-1">
      <CardButton v-if="buttonProps" v-bind="buttonProps" />
      <CardButton v-if="secondaryButtonProps" variant="outline" v-bind="secondaryButtonProps" />
      <CardCartButton v-if="cartButtonProps" v-bind="cartButtonProps" />
      <CardFavoriteButton v-if="favoriteButtonProps" v-bind="favoriteButtonProps" />
    </div>

    <div v-if="trustBadges.length > 0" class="grid gap-2 pt-1 sm:grid-cols-3">
      <div
        v-for="b in trustBadges"
        :key="b.text"
        class="flex items-center gap-2 rounded-lg border border-blender-border/70 bg-blender-surface/50 px-3 py-2 text-[11px] text-zinc-300"
      >
        <Icon v-if="b.icon" :name="b.icon" className="size-4 text-blue-400" />
        <span>{{ b.text }}</span>
      </div>
    </div>

    <CardTags v-if="tagsProps" v-bind="tagsProps" />

    <div v-if="inlineTabsProps" class="border-t border-blender-border pt-4">
      <ProductTabs v-bind="inlineTabsProps" />
    </div>
  </div>
</template>
