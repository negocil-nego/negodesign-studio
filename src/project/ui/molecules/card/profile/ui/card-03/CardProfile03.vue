<script setup lang="ts">
import { StyleComponent } from "../../../../../../data/style.component";
import CardActionsButton from "../../../shared/card-actions-button/ui/CardActionsButton.vue";
import CardDescription from "../../../shared/card-description/ui/CardDescription.vue";
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
    handleProps,
    videoProps,
    imageProps,
    thumbnailVideoProps,
    descriptionProps,
    tagsProps,
    actionsProps,
    editActionProps,
} = defineProps<CardProfileBaseProps>();

const style = StyleComponent.card.profile.v03;
</script>

<template>
    <aside :class="style.root">
        <!-- Cover -->
        <div :class="style.cover">
            <CardThumbnailVideo v-if="thumbnailVideoProps" v-bind="thumbnailVideoProps" />
            <CardVideo v-else-if="videoProps" v-bind="videoProps" :controls="false" />
            <CardImage v-else-if="imageProps" v-bind="imageProps" :class-name="style.coverMedia" />
            <div v-else :class="style.coverFallback" />

            <!-- Edit Profile button — top-right of cover -->
            <div v-if="editActionProps" :class="style.editAction">
                <CardActionsButton :actions="editActionProps.actions" variant="icon-outline" />
            </div>

            <!-- Avatar anchored to bottom of cover -->
            <div v-if="logoProps" :class="style.logoWrapper">
                <CardLogo v-bind="logoProps" :class-name="style.logoImage" />
            </div>
        </div>

        <!-- Body -->
        <div :class="style.body">
            <!-- Name + handle row alongside Follow / Message buttons -->
            <div :class="style.headRow">
                <div :class="style.nameGroup">
                    <CardTitle v-if="titleProps" v-bind="titleProps" :class-name="style.name" />
                    <span v-if="handleProps" :class="style.handle">{{ handleProps }}</span>
                </div>

                <div v-if="actionsProps" :class="style.actionsGroup">
                    <CardActionsButton :actions="actionsProps.actions" variant="mixed" />
                </div>
            </div>

            <!-- About / Description -->
            <div v-if="descriptionProps || tagsProps" :class="style.aboutSection">
                <span :class="style.aboutLabel">About</span>
                <CardDescription v-if="descriptionProps" v-bind="descriptionProps" :class-name="style.description" />
            </div>

            <!-- Stats -->
            <CardTags
                v-if="tagsProps"
                v-bind="tagsProps"
                variant="stats"
                :class-name="style.stats"
                :item-class-name="style.statItem"
                :title-class-name="style.statValue"
                :description-class-name="style.statLabel"
            />
        </div>
    </aside>
</template>

