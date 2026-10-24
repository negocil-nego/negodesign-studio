<script lang="ts">
    import { StyleComponent } from "../../../../../../data/style.component";
    import CardButton from "../../../shared/card-button/ui/CardButton.svelte";
    import CardCartButton from "../../../shared/card-cart-button/ui/CardCartButton.svelte";
    import CardDescription from "../../../shared/card-description/ui/CardDescription.svelte";
    import CardFavoriteButton from "../../../shared/card-favorite-button/ui/CardFavoriteButton.svelte";
    import CardImage from "../../../shared/card-image/ui/CardImage.svelte";
    import CardLogo from "../../../shared/card-logo/ui/CardLogo.svelte";
    import CardThumbnailVideo from "../../../shared/card-thumbnail-video/ui/CardThumbnailVideo.svelte";
    import CardTitle from "../../../shared/card-title/ui/CardTitle.svelte";
    import CardVideo from "../../../shared/card-video/ui/CardVideo.svelte";
    import CardTags from "../../../shared/card-tags/ui/CardTags.svelte";
    import CardActionsButton from "../../../shared/card-actions-button/ui/CardActionsButton.svelte";
    import type { CardProfileBaseProps } from "../../data/types";

    let {
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
    }: CardProfileBaseProps = $props();

    const style = StyleComponent.card.profile.v01;
</script>

<aside class={style.root}>
    <div class={style.cover}>
        <div class={StyleComponent.card.profile.fill}>
            {#if thumbnailVideoProps}
                <CardThumbnailVideo {...thumbnailVideoProps}/>
            {:else if videoProps}
                <CardVideo {...videoProps} controls={false}/>
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

        <div class={style.logo}>
            {#if logoProps}
                <div class={style.logoFrame}>
                    <CardLogo {...logoProps} className={style.logoImage} />
                </div>
            {/if}
        </div>
    </div>

    <div class={style.body}>
        <div class={style.titleGroup}>
            {#if titleProps}
                <CardTitle {...titleProps} />
            {/if}
        </div>

        {#if tagsProps}
            <CardTags {...tagsProps} />
        {/if}

        {#if actionsProps}
            <CardActionsButton {...actionsProps} />
        {/if}

        <div class={style.descriptionGroup}>
            {#if descriptionProps}
                <CardDescription {...descriptionProps} />
            {/if}
        </div>

        {#if buttonProps}
            <div class={style.footer}>
                <CardButton {...buttonProps} />
            </div>
        {/if}
    </div>
</aside>
