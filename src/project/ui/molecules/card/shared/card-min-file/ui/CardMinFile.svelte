<script lang="ts">
    import { StyleComponent } from "../../../../../../data/style.component";
    import CardImage from "../../card-image/ui/CardImage.svelte";
    import type { CardMinFileProps } from "../data/types";

    let { imageProps, active = false, onClick }: CardMinFileProps = $props();
</script>

<button
    type="button"
    class={`${StyleComponent.card.cardMinFile.root} ${active ? StyleComponent.card.cardMinFile.active : ""}`}
    onclick={onClick}
>
    {#if imageProps}
        <CardImage {...imageProps} />
    {/if}
</button>
