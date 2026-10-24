<script lang="ts">
    import { StyleComponent } from "../../../../../../data/style.component";
    import CardButton from "../../../shared/card-button/ui/CardButton.svelte";
    import CardCartButton from "../../../shared/card-cart-button/ui/CardCartButton.svelte";
    import CardDescription from "../../../shared/card-description/ui/CardDescription.svelte";
    import CardFavoriteButton from "../../../shared/card-favorite-button/ui/CardFavoriteButton.svelte";
    import CardImage from "../../../shared/card-image/ui/CardImage.svelte";
    import CardLogo from "../../../shared/card-logo/ui/CardLogo.svelte";
    import CardMinFile from "../../../shared/card-min-file/ui/CardMinFile.svelte";
    import CardPrice from "../../../shared/card-price/ui/CardPrice.svelte";
    import CardRating from "../../../shared/card-rating/ui/CardRating.svelte";
    import CardThumbnailVideo from "../../../shared/card-thumbnail-video/ui/CardThumbnailVideo.svelte";
    import CardTitle from "../../../shared/card-title/ui/CardTitle.svelte";
    import CardVideo from "../../../shared/card-video/ui/CardVideo.svelte";
    import type { CardProductBaseProps } from "../../data/types";

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
        minFileProps,
    }: CardProductBaseProps = $props();

    const style = StyleComponent.card.product.v01;
</script>

<aside class={style.root}>
    <div class={style.media}>
        <div class={`${StyleComponent.card.product.frame} ${StyleComponent.card.product.fill}`}>
            {#if thumbnailVideoProps}
                <CardThumbnailVideo {...thumbnailVideoProps} />
            {:else if videoProps}
                <CardVideo {...videoProps} controls={false} />
            {:else if imageProps}
                <CardImage {...imageProps} />
            {/if}
        </div>

        <div class={style.actions}>
            {#if cartButtonProps}
                <CardCartButton {...cartButtonProps} />
            {/if}
            {#if favoriteButtonProps}
                <CardFavoriteButton {...favoriteButtonProps} />
            {/if}
        </div>

        {#if priceProps}
            <div class={style.priceBadge}>
                <CardPrice {...priceProps} />
            </div>
        {/if}
    </div>

    <div class={style.body}>
        <div class={style.header}>
            {#if logoProps}
                <CardLogo {...logoProps} />
            {/if}
            {#if titleProps}
                <div class={style.titleGroup}>
                    <CardTitle {...titleProps} />
                    {#if ratingProps}
                        <CardRating {...ratingProps} />
                    {:else}
                        <CardRating rating={0} />
                    {/if}
                </div>
            {/if}
        </div>

        {#if minFileProps?.length}
            <div class={style.thumbs}>
                {#each minFileProps as minFile}
                    <CardMinFile {...minFile} />
                {/each}
            </div>
        {/if}

        {#if descriptionProps}
            <CardDescription {...descriptionProps} />
        {/if}

        {#if buttonProps}
            <div class={style.footer}>
                <CardButton {...buttonProps} />
            </div>
        {/if}
    </div>
</aside>
