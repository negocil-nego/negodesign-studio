<script setup lang="ts">
import { StyleComponent } from "../../../../../../data/style.component";
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

const style = StyleComponent.card.product.v01;
</script>

<template>
    <aside :class="style.root">
        <div :class="style.media">
            <div :class="`${StyleComponent.card.product.frame} ${StyleComponent.card.product.fill}`">
                <CardThumbnailVideo v-if="thumbnailVideoProps" v-bind="thumbnailVideoProps" />
                <CardVideo v-else-if="videoProps" v-bind="videoProps" :controls="false" />
                <CardImage v-else-if="imageProps" v-bind="imageProps" />
            </div>

            <div :class="style.actions">
                <CardCartButton v-if="cartButtonProps" v-bind="cartButtonProps" />
                <CardFavoriteButton v-if="favoriteButtonProps" v-bind="favoriteButtonProps" />
            </div>

            <div v-if="priceProps" :class="style.priceBadge">
                <CardPrice v-bind="priceProps" />
            </div>
        </div>

        <div :class="style.body">
            <div :class="style.header">
                <CardLogo v-if="logoProps" v-bind="logoProps" />
                <div v-if="titleProps" :class="style.titleGroup">
                    <CardTitle v-bind="titleProps" />
                    <CardRating v-if="ratingProps" v-bind="ratingProps" />
                    <CardRating v-else :rating="0" />
                </div>
            </div>

            <div v-if="minFileProps?.length" :class="style.thumbs">
                <CardMinFile v-for="(minFile, index) in minFileProps" :key="index" v-bind="minFile" />
            </div>

            <CardDescription v-if="descriptionProps" v-bind="descriptionProps" />

            <div v-if="buttonProps" :class="style.footer">
                <CardButton v-bind="buttonProps" />
            </div>
        </div>
    </aside>
</template>
