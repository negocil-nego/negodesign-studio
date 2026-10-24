<script lang="ts">
    import CardButton from "../../../shared/card-button/ui/CardButton.svelte";
    import CardCartButton from "../../../shared/card-cart-button/ui/CardCartButton.svelte";
    import CardDescription from "../../../shared/card-description/ui/CardDescription.svelte";
    import CardFavoriteButton from "../../../shared/card-favorite-button/ui/CardFavoriteButton.svelte";
    import CardImage from "../../../shared/card-image/ui/CardImage.svelte";
    import CardLogo from "../../../shared/card-logo/ui/CardLogo.svelte";
    import CardRating from "../../../shared/card-rating/ui/CardRating.svelte";
    import CardThumbnailVideo from "../../../shared/card-thumbnail-video/ui/CardThumbnailVideo.svelte";
    import CardTitle from "../../../shared/card-title/ui/CardTitle.svelte";
    import CardVideo from "../../../shared/card-video/ui/CardVideo.svelte";
    import type { CardProfileBaseProps } from "../../data/types";

    let {
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
    }: CardProfileBaseProps = $props();
</script>

<aside class="blender-panel p-6 rounded-xl border border-gray-100 shadow-sm">
    <div class="flex justify-between items-start mb-6">
        <div class="flex gap-4 items-center">
            {#if logoProps}
                <CardLogo {...logoProps} />
            {/if}
            <div class="flex flex-col gap-1">
                {#if titleProps}
                    <CardTitle {...titleProps} />
                {/if}
                {#if ratingProps}
                    <CardRating {...ratingProps} />
                {/if}
            </div>
        </div>
        <div class="flex gap-2">
            {#if cartButtonProps}
                <CardCartButton {...cartButtonProps} />
            {/if}
            {#if favoriteButtonProps}
                <CardFavoriteButton {...favoriteButtonProps} />
            {/if}
        </div>
    </div>

    <div class="flex flex-col gap-4">
        {#if descriptionProps}
            <CardDescription {...descriptionProps} />
        {/if}
        
        <div class="rounded-lg overflow-hidden border border-gray-100">
            {#if thumbnailVideoProps}
                <CardThumbnailVideo {...thumbnailVideoProps} />
            {:else if videoProps}
                <CardVideo {...videoProps} />
            {:else if imageProps}
                <CardImage {...imageProps} />
            {/if}
        </div>
    </div>

    {#if buttonProps}
        <div class="mt-6">
            <CardButton {...buttonProps} />
        </div>
    {/if}
</aside>
