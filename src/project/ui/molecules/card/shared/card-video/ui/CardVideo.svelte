<script lang="ts">
    import { onMount } from "svelte";
    import { StyleComponent } from "../../../../../../data/style.component";
    import type { CardVideoProps } from "../data/types";

    let { src, poster, preload = "metadata", controls = false }: CardVideoProps = $props();

    let video: HTMLVideoElement | undefined;

    function playVideo() {
        if (!video) return;
        // O WebKit ignora por vezes o atributo `muted`; forçar a propriedade é o que permite o autoplay
        video.muted = true;
        video.play().catch(() => {});
    }

    onMount(playVideo);
</script>

<video
    bind:this={video}
    {src}
    {poster}
    {preload}
    {controls}
    autoplay
    muted
    loop
    playsinline
    oncanplay={playVideo}
    class={StyleComponent.card.media.default}
></video>
