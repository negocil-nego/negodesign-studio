<script lang="ts">
    import Icon from "../../../../../atoms/icon/ui/Icon.svelte";
    import { StyleComponent } from "../../../../../../data/style.component";
    import type { CardRatingProps } from "../data/types";

    let { rating }: CardRatingProps = $props();

    const MAX_STARS = 5;
    let filled = $derived(Math.min(MAX_STARS, Math.max(0, Math.round(rating))));
</script>

<div
    class={StyleComponent.card.rating.root}
    role="img"
    aria-label={`rating ${filled} de ${MAX_STARS}`}
>
    {#each Array(MAX_STARS) as _, index}
        <Icon
            name="Star"
            className={index < filled ? StyleComponent.card.rating.filled : ""}
        />
    {/each}
</div>