<script setup lang="ts">
import CardActionsButton from "../../../shared/card-actions-button/ui/CardActionsButton.vue";
import CardButton from "../../../shared/card-button/ui/CardButton.vue";
import CardCartButton from "../../../shared/card-cart-button/ui/CardCartButton.vue";
import CardDescription from "../../../shared/card-description/ui/CardDescription.vue";
import CardFavoriteButton from "../../../shared/card-favorite-button/ui/CardFavoriteButton.vue";
import CardImage from "../../../shared/card-image/ui/CardImage.vue";
import CardLogo from "../../../shared/card-logo/ui/CardLogo.vue";
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
    <aside class="w-72 rounded-3xl bg-white dark:bg-blender-card border border-blender-border transition-colors overflow-hidden shadow-xl">
        <div class="relative h-32 w-full">
            <div class="h-40 w-full rounded-md object-cover">
                <CardThumbnailVideo v-if="thumbnailVideoProps" v-bind="thumbnailVideoProps" />
                <CardVideo v-else-if="videoProps" v-bind="videoProps" :controls="false" />
                <CardImage v-else-if="imageProps" v-bind="imageProps" />
            </div>

            <div class="absolute top-2 flex justify-between w-full gap-2 z-10 p-2">
                <CardCartButton v-if="cartButtonProps" v-bind="cartButtonProps" />
                <CardFavoriteButton v-if="favoriteButtonProps" v-bind="favoriteButtonProps" />
            </div>

            <div class="absolute left-1/2 -translate-x-1/2 -translate-y-1/2 z-10">
                <div v-if="logoProps" class="rounded-full overflow-hidden border-4 border-white bg-white shadow-md">
                    <CardLogo v-bind="logoProps" class="w-20 h-20 rounded-full" />
                </div>
            </div>
        </div>

        <div class="px-6 pt-14 pb-6 flex flex-col gap-4">
            <div class="flex flex-col items-center gap-1 z-20 border-white text-white bg-black/5 p-2 font-bold">
                <CardTitle v-if="titleProps" v-bind="titleProps" />
            </div>

            <CardTags v-if="tagsProps" v-bind="tagsProps" />

            <CardActionsButton v-if="actionsProps" v-bind="actionsProps" />

            <div class="flex flex-col gap-3">
                <CardDescription v-if="descriptionProps" v-bind="descriptionProps" />
            </div>

            <div v-if="buttonProps" class="mt-2">
                <CardButton v-bind="buttonProps" />
            </div>
        </div>
    </aside>
</template>
