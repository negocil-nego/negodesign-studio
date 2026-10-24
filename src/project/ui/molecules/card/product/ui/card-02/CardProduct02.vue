<script setup lang="ts">
import CardButton from "../../../shared/card-button/ui/CardButton.vue";
import CardCartButton from "../../../shared/card-cart-button/ui/CardCartButton.vue";
import CardDescription from "../../../shared/card-description/ui/CardDescription.vue";
import CardFavoriteButton from "../../../shared/card-favorite-button/ui/CardFavoriteButton.vue";
import CardImage from "../../../shared/card-image/ui/CardImage.vue";
import CardLogo from "../../../shared/card-logo/ui/CardLogo.vue";
import CardMinFile from "../../../shared/card-min-file/ui/CardMinFile.vue";
import CardPrice from "../../../shared/card-price/ui/CardPrice.vue";
import CardRating from "../../../shared/card-rating/ui/CardRating.vue";
import CardThumbnailVideo from "../../../shared/card-thumbnail-video/ui/CardThumbnailVideo.vue";
import CardTitle from "../../../shared/card-title/ui/CardTitle.vue";
import CardVideo from "../../../shared/card-video/ui/CardVideo.vue";
import type { CardProductBaseProps } from "../../data/types";

const {
    logoProps,
    titleProps,
    videoProps,
    imageProps,
    priceProps,
    ratingProps,
    buttonProps,
    cartButtonProps,
    descriptionProps,
    favoriteButtonProps,
    thumbnailVideoProps,
    minFileProps,
} = defineProps<CardProductBaseProps>();
</script>

<template>
    <aside class="w-100 flex gap-4 p-5 rounded-2xl bg-white dark:bg-blender-card border border-blender-border shadow-sm transition-colors">
        <div class="flex flex-col gap-2 w-44 shrink-0">
            <div class="relative h-40 w-full">
                <div class="overflow-hidden rounded-md border border-gray-100 h-full w-full">
                    <CardThumbnailVideo v-if="thumbnailVideoProps" v-bind="thumbnailVideoProps" />
                    <CardVideo v-else-if="videoProps" v-bind="videoProps" :controls="false" />
                    <CardImage v-else-if="imageProps" v-bind="imageProps" />
                </div>

                <div class="absolute top-2 inset-x-0 flex justify-between gap-2 z-10 p-2">
                    <CardCartButton v-if="cartButtonProps" v-bind="cartButtonProps" />
                    <CardFavoriteButton v-if="favoriteButtonProps" v-bind="favoriteButtonProps" />
                </div>
            </div>

            <div v-if="minFileProps?.length" class="flex gap-2 overflow-x-auto">
                <CardMinFile v-for="(minFile, index) in minFileProps" :key="index" v-bind="minFile" />
            </div>
        </div>

        <div class="flex flex-col gap-3 flex-1 min-w-0">
            <div class="flex items-center gap-3">
                <CardLogo v-if="logoProps" v-bind="logoProps" />
                <div v-if="titleProps" class="flex flex-col gap-1 min-w-0">
                    <CardTitle v-bind="titleProps" />
                    <CardRating v-if="ratingProps" v-bind="ratingProps" />
                    <CardRating v-else :rating="0" />
                </div>
            </div>

            <CardPrice v-if="priceProps" v-bind="priceProps" />

            <CardDescription v-if="descriptionProps" v-bind="descriptionProps" />

            <div v-if="buttonProps" class="mt-auto">
                <CardButton v-bind="buttonProps" />
            </div>
        </div>
    </aside>
</template>
