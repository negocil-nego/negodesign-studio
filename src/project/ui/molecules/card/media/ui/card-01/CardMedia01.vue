<script setup lang="ts">
import CardButton from "../../../shared/card-button/ui/CardButton.vue";
import CardCartButton from "../../../shared/card-cart-button/ui/CardCartButton.vue";
import CardDescription from "../../../shared/card-description/ui/CardDescription.vue";
import CardFavoriteButton from "../../../shared/card-favorite-button/ui/CardFavoriteButton.vue";
import CardImage from "../../../shared/card-image/ui/CardImage.vue";
import CardLogo from "../../../shared/card-logo/ui/CardLogo.vue";
import CardPrice from "../../../shared/card-price/ui/CardPrice.vue";
import CardRating from "../../../shared/card-rating/ui/CardRating.vue";
import CardThumbnailVideo from "../../../shared/card-thumbnail-video/ui/CardThumbnailVideo.vue";
import CardTitle from "../../../shared/card-title/ui/CardTitle.vue";
import CardVideo from "../../../shared/card-video/ui/CardVideo.vue";
import type { CardMediaBaseProps } from "../../data/types";

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
} = defineProps<CardMediaBaseProps>();
</script>

<template>
    <aside class="w-87.5 p-5 rounded-lg bg-white dark:bg-blender-card border border-blender-border transition-colors">
        <div class="flex justify-between">
            <div class="flex gap-2">
                <CardLogo v-if="logoProps" v-bind="logoProps" />
                <div v-if="titleProps || ratingProps || priceProps" class="flex flex-col gap-2">
                    <CardTitle v-if="titleProps" v-bind="titleProps" />
                    <CardRating v-if="ratingProps" v-bind="ratingProps" />
                    <CardRating v-else :rating="0" />
                    <CardPrice v-if="priceProps" v-bind="priceProps" />
                </div>
            </div>
            <div v-if="cartButtonProps || favoriteButtonProps" class="flex gap-2">
                <CardCartButton v-if="cartButtonProps" v-bind="cartButtonProps" />
                <CardFavoriteButton v-if="favoriteButtonProps" v-bind="favoriteButtonProps" />
            </div>
        </div>

        <div class="h-full">
            <div class="my-4">
                <CardDescription v-if="descriptionProps" v-bind="descriptionProps" />
            </div>
            <div class="overflow-hidden rounded-md border border-gray-100 h-full w-full">
                <CardThumbnailVideo v-if="thumbnailVideoProps" v-bind="thumbnailVideoProps" :visible-button-play="true" />
                <CardVideo v-else-if="videoProps" v-bind="videoProps" />
                <CardImage v-else-if="imageProps" v-bind="imageProps" />
            </div>
        </div>

        <div v-if="buttonProps" class="my-2">
            <CardButton v-bind="buttonProps" />
        </div>
    </aside>
</template>
