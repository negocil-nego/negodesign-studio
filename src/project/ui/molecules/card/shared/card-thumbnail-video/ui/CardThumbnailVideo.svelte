<script lang="ts">
    import CardImage from "../../card-image/ui/CardImage.svelte";
    import CardVideo from "../../card-video/ui/CardVideo.svelte";
    import type { CardThumbnailVideoProps } from "../data/types";

    let { imageUrl, videoUrl }: CardThumbnailVideoProps = $props();

    let isHoverImage = $state(false);
</script>

<div
    role="presentation"
    onmouseenter={() => (isHoverImage = true)}
    onmouseleave={() => (isHoverImage = false)}
>
    {#if isHoverImage}
        <CardVideo src={videoUrl} />
    {:else}
        <CardImage src={imageUrl} />
    {/if}
</div>