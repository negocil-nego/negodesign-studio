<script setup lang="ts">
import Progress from "../../../../../atoms/progress/ui/Progress.vue";
import CardButton from "../../../../../molecules/card/shared/card-button/ui/CardButton.vue";
import CardDescription from "../../../../../molecules/card/shared/card-description/ui/CardDescription.vue";
import CardImage from "../../../../../molecules/card/shared/card-image/ui/CardImage.vue";
import CardRating from "../../../../../molecules/card/shared/card-rating/ui/CardRating.vue";
import CardTitle from "../../../../../molecules/card/shared/card-title/ui/CardTitle.vue";
import type { ProductRatingProps } from "../data/types";

withDefaults(defineProps<ProductRatingProps>(), {
  summaryLayout: "bars-left",
  averageScore: "4.5/5",
  ratingProps: () => ({ rating: 5 }),
  totalReviewsText: "150 Reviews",
  recommendedLabel: "Recommended",
  breakdown: () => [
    { stars: 5, label: "5 Star", percentage: 78 },
    { stars: 4, label: "4 Star", percentage: 52 },
    { stars: 3, label: "3 Star", percentage: 28 },
    { stars: 2, label: "2 Star", percentage: 14 },
    { stars: 1, label: "1 Star", percentage: 6 },
  ],
  filterTitle: "REFINED BY",
  reviews: () => [],
  className: "",
});
</script>

<template>
  <section :class="`space-y-6 ${className}`">
    <CardTitle v-if="titleProps" v-bind="titleProps" />

    <div class="rounded-2xl border border-blender-border bg-blender-surface/50 p-5">
      <div class="grid gap-6 md:grid-cols-12 md:items-center">
        <div class="space-y-2 md:col-span-5">
          <div
            v-for="row in breakdown"
            :key="row.stars"
            class="grid grid-cols-[3rem_minmax(0,1fr)] items-center gap-2.5 text-xs"
          >
            <span class="font-medium text-zinc-400">{{ row.label ?? `${row.stars} ★` }}</span>
            <Progress :value="row.percentage" variant="warning" />
          </div>
        </div>
        <div class="flex items-center gap-6 md:col-span-4">
          <div>
            <div class="text-2xl font-bold text-zinc-100">{{ averageScore }}</div>
            <CardRating v-if="ratingProps" v-bind="ratingProps" />
            <p class="text-[11px] text-zinc-400">{{ totalReviewsText }}</p>
          </div>
          <div v-if="recommendedPercent" class="border-l border-blender-border pl-6">
            <div class="text-xl font-bold text-zinc-100">{{ recommendedPercent }}</div>
            <p class="text-[11px] text-zinc-400">{{ recommendedLabel }}</p>
          </div>
        </div>
        <div v-if="writeReviewButtonProps" class="md:col-span-3 md:text-right">
          <CardButton v-bind="writeReviewButtonProps" />
        </div>
      </div>
    </div>

    <div v-if="reviews.length > 0" class="divide-y divide-blender-border/60">
      <article v-for="(review, idx) in reviews" :key="review.id ?? idx" class="space-y-3 py-5">
        <div class="flex items-start justify-between gap-3">
          <div class="flex items-center gap-3">
            <img
              :src="review.avatarSrc ?? '/favicon.png'"
              :alt="review.author"
              class="size-9 rounded-full border border-blender-border bg-blender-surface object-cover"
            />
            <div>
              <span class="text-xs font-semibold text-zinc-100">{{ review.author }}</span>
              <CardRating v-if="review.ratingProps" v-bind="review.ratingProps" />
            </div>
          </div>
          <span v-if="review.date" class="text-[11px] text-zinc-500">{{ review.date }}</span>
        </div>
        <h5 v-if="review.headline" class="text-xs font-semibold text-zinc-100">{{ review.headline }}</h5>
        <CardDescription v-if="review.descriptionProps" v-bind="review.descriptionProps" />
        <div v-if="review.mediaImages?.length" class="flex flex-wrap gap-2.5 pt-1">
          <div
            v-for="(img, i) in review.mediaImages"
            :key="i"
            class="size-20 overflow-hidden rounded-xl border border-blender-border bg-blender-surface"
          >
            <CardImage v-bind="img" className="h-full w-full object-cover" />
          </div>
        </div>
      </article>
    </div>
  </section>
</template>
