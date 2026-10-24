<script setup lang="ts">
import CardActionsButton from "../../../shared/card-actions-button/ui/CardActionsButton.vue";
import CardButton from "../../../shared/card-button/ui/CardButton.vue";
import CardCartButton from "../../../shared/card-cart-button/ui/CardCartButton.vue";
import CardDescription from "../../../shared/card-description/ui/CardDescription.vue";
import CardFavoriteButton from "../../../shared/card-favorite-button/ui/CardFavoriteButton.vue";
import CardImage from "../../../shared/card-image/ui/CardImage.vue";
import CardLogo from "../../../shared/card-logo/ui/CardLogo.vue";
import CardRating from "../../../shared/card-rating/ui/CardRating.vue";
import CardTags from "../../../shared/card-tags/ui/CardTags.vue";
import CardThumbnailVideo from "../../../shared/card-thumbnail-video/ui/CardThumbnailVideo.vue";
import CardTitle from "../../../shared/card-title/ui/CardTitle.vue";
import CardVideo from "../../../shared/card-video/ui/CardVideo.vue";
import type { CardProfileBaseProps } from "../../data/types";

const {
    logoProps,
    titleProps,
    videoProps,
    imageProps,
    ratingProps,
    buttonProps,
    cartButtonProps,
    descriptionProps,
    favoriteButtonProps,
    thumbnailVideoProps,
    tagsProps,
    actionsProps,
} = defineProps<CardProfileBaseProps>();
</script>

<template>
    <aside class="w-72 rounded-3xl bg-white dark:bg-blender-card border border-blender-border transition-colors overflow-hidden shadow-sm">
        <div class="h-44 w-full relative">
            <div class="h-40 w-full rounded-md object-cover">
                <CardThumbnailVideo v-if="thumbnailVideoProps" v-bind="thumbnailVideoProps" />
                <CardVideo v-else-if="videoProps" v-bind="videoProps" :controls="false" />
                <CardImage v-else-if="imageProps" v-bind="imageProps" />
            </div>

            <div class="absolute top-2 flex justify-end w-full gap-2 z-10 p-2">
                <CardCartButton v-if="cartButtonProps" v-bind="cartButtonProps" />
                <CardFavoriteButton v-if="favoriteButtonProps" v-bind="favoriteButtonProps" />
            </div>
        </div>

        <div class="px-6 pb-6 flex flex-col gap-4">
            <div class="relative">
                <div v-if="logoProps" class="w-24 h-24 rounded-full border-4 border-white overflow-hidden bg-white shadow-md absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 z-10">
                    <CardLogo v-bind="logoProps" class="w-full h-full object-cover" />
                </div>
            </div>

            <CardTitle v-if="titleProps" v-bind="titleProps" class="text-2xl font-bold text-gray-900 mt-8" />

            <CardRating v-if="ratingProps" v-bind="ratingProps" />

            <CardTags v-if="tagsProps" v-bind="tagsProps" />

            <CardActionsButton v-if="actionsProps" v-bind="actionsProps" />

            <CardDescription v-if="descriptionProps" v-bind="descriptionProps" />

            <CardButton v-if="buttonProps" v-bind="buttonProps" />
        </div>
    </aside>
</template>
