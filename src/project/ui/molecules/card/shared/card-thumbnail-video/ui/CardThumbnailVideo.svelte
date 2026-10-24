<script lang="ts">
    import { StyleComponent } from "../../../../../../data/style.component";
    import Icon from "../../../../../atoms/icon/ui/Icon.svelte";
    import CardImage from "../../card-image/ui/CardImage.svelte";
    import CardVideo from "../../card-video/ui/CardVideo.svelte";
    import type { CardThumbnailVideoProps } from "../data/types";

    let {
        videoProps,
        imageProps,
        visibleButtonPlay = false,
    }: CardThumbnailVideoProps = $props();

    let isHoverImage = $state(false);
</script>

<div
    class={StyleComponent.card.cardThumbnailVideo.container}
    role="presentation"
    onmouseenter={() => (isHoverImage = true)}
    onmouseleave={() => (isHoverImage = false)}
>
    <!-- Imagem pré-carregada (loading eager) — define a altura e fica sempre por baixo -->
    <CardImage {...imageProps} loading="eager" />

    <!--
        Vídeo montado desde o início com preload="auto": ao passar o rato já está
        pré-carregado e a reproduzir, pelo que o hover apenas o torna visível.
    -->
    <div
        class={StyleComponent.card.cardThumbnailVideo.video}
        class:invisible={!isHoverImage}
    >
        <CardVideo {...videoProps} preload="auto" />
    </div>

    {#if visibleButtonPlay}
        <div
            class={StyleComponent.card.cardThumbnailVideo.playButton}
            class:invisible={isHoverImage}
        >
            <Icon
                name="Play"
                className={StyleComponent.card.cardThumbnailVideo.playIcon}
            />
        </div>
    {/if}
</div>
