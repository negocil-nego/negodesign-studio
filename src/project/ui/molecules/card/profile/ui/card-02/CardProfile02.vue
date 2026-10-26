<script setup lang="ts">
import { StyleComponent } from "../../../../../../data/style.component";
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

const style = StyleComponent.card.profile.v02;
</script>

<template>
    <aside :class="style.root">
        <div :class="style.cover">
            <div :class="StyleComponent.card.profile.fill">
                <CardThumbnailVideo v-if="thumbnailVideoProps" v-bind="thumbnailVideoProps" />
                <CardVideo v-else-if="videoProps" v-bind="videoProps" :controls="false" />
                <CardImage v-else-if="imageProps" v-bind="imageProps" />
            </div>

            <div :class="style.actions">
                <CardCartButton v-if="cartButtonProps" v-bind="cartButtonProps" />
                <CardFavoriteButton v-if="favoriteButtonProps" v-bind="favoriteButtonProps" />
            </div>
        </div>

        <div :class="style.body">
            <div class="relative">
                <div v-if="logoProps" :class="style.logoFrame">
                    <CardLogo v-bind="logoProps" :class-name="style.logoImage" />
                </div>
            </div>

            <CardTitle v-if="titleProps" v-bind="titleProps" :class-name="style.infoTitle" />

            <CardRating v-if="ratingProps" v-bind="ratingProps" />

            <CardTags v-if="tagsProps" v-bind="tagsProps" />

            <CardActionsButton v-if="actionsProps" v-bind="actionsProps" />

            <CardDescription v-if="descriptionProps" v-bind="descriptionProps" />

            <CardButton v-if="buttonProps" v-bind="buttonProps" />
        </div>
    </aside>
</template>
