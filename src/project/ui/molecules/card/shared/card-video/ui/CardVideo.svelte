<script lang="ts">
    import type { CardVideoProps } from "../data/types";

    let { src }: CardVideoProps = $props();
</script>

<video
    src={src}
    preload="metadata"
    class="h-full w-full rounded-md object-cover transition-opacity duration-500"
    muted
    loop
    playsinline
></video>
