<script lang="ts">
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
    }: CardProfileBaseProps = $props();
</script>

<aside class="w-96 rounded-3xl blender-panel overflow-hidden bg-white shadow-xl border border-gray-100">
    <div class="relative h-48 w-full bg-amber-950">
        <div class="h-full w-full">
            {#if thumbnailVideoProps}
                <CardThumbnailVideo {...thumbnailVideoProps} />
            {:else if videoProps}
                <CardVideo {...videoProps} />
            {:else if imageProps}
                <CardImage {...imageProps} />
            {/if}
        </div>

        <div class="absolute top-4 right-4 flex gap-2 z-10">
            {#if cartButtonProps}
                <CardCartButton {...cartButtonProps} />
            {/if}
            {#if favoriteButtonProps}
                <CardFavoriteButton {...favoriteButtonProps} />
            {/if}
        </div>

        <div class="absolute -bottom-12 left-6 z-20">
            {#if logoProps}
                <div class="rounded-full border-4 border-white shadow-lg overflow-hidden bg-white">
                    <CardLogo {...logoProps} className="w-32 h-32 rounded-full" />
                </div>
            {/if}
        </div>
    </div>

    <div class="px-8 pt-16 pb-8 flex flex-col gap-6">
        <div class="flex justify-end -mt-10 mb-2">
            <span class="bg-green-50 text-green-600 px-4 py-1.5 rounded-full text-xs font-bold flex items-center gap-2 border border-green-100">
                <span class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></span>
                Open to work
            </span>
        </div>

        <div class="flex flex-col gap-1">
            {#if titleProps}
                <CardTitle {...titleProps} />
            {/if}
            <span class="text-sm text-gray-400 font-medium">UX Lead | Design Manager</span>
        </div>

        {#if tagsProps}
            <CardTags {...tagsProps} />
        {/if}

        <div class="flex flex-col gap-3">
            <span class="text-sm font-extrabold text-gray-900 uppercase tracking-widest">About</span>
            {#if descriptionProps}
                <CardDescription {...descriptionProps} />
            {/if}
        </div>

        {#if buttonProps}
            <div class="mt-2">
                <CardButton {...buttonProps} />
            </div>
        {/if}
    </div>
</aside>
