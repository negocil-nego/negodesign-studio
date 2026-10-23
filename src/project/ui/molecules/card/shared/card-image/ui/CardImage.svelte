<script lang="ts">
    import type { CardImageProps } from "../data/types";

    let { src, alt }: CardImageProps = $props();
</script>

<img
    {src}
    loading="lazy"
    decoding="async"
    {alt}
    class="h-full w-full rounded-md object-cover transition-opacity duration-500"
/>
