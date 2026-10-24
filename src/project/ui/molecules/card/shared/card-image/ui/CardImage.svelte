<script lang="ts">
    import { StyleComponent } from "../../../../../../data/style.component";
    import type { CardImageProps } from "../data/types";

    let { src, alt, srcset, sizes, loading = "lazy", fetchpriority }: CardImageProps = $props();
</script>

<img
    {src}
    {srcset}
    {sizes}
    {alt}
    {fetchpriority}
    {loading}
    decoding="async"
    class={`${StyleComponent.card.media.default} transition-opacity duration-500`}
/>
