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
    import type { CardMediaBaseProps } from "../../data/types";

    let {
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
    }: CardMediaBaseProps = $props();
</script>

<aside>
    <div class="flex justify-between">
        <div>
            {#if logoProps}
                <CardLogo {...logoProps} />
            {/if}
            {#if titleProps || ratingProps}
                <div class="flex flex-col gap-2">
                    {#if titleProps}
                        <CardTitle {...titleProps} />
                    {/if}
                    {#if ratingProps}
                        <CardRating {...ratingProps} />
                    {/if}
                </div>
            {/if}
        </div>
        <div class="flex flex-col gap-2">
            {#if cartButtonProps}
                <CardCartButton {...cartButtonProps} />
            {/if}
            {#if favoriteButtonProps}
                <CardFavoriteButton {...favoriteButtonProps} />
            {/if}
        </div>
    </div>

    <div>
        <div>
        {#if descriptionProps}
            <CardDescription {...descriptionProps} />
        {/if}
        </div>
        <div>
            {#if thumbnailVideoProps}
                <CardThumbnailVideo {...thumbnailVideoProps} />
            {:else if videoProps}
                <CardVideo {...videoProps} />
            {:else if imageProps}
                <CardImage {...imageProps} />
            {/if}
        </div>
    </div>

    <div>
        {#if buttonProps}
            <CardButton {...buttonProps} />
        {/if}
    </div>
</aside>
