<script lang="ts">
    import { StyleComponent } from "../../../../../../data/style.component";
    import type { CardPriceProps } from "../data/types";

    let { number }: CardPriceProps = $props();
</script>
<div class={StyleComponent.card.price.root} id="card-price">
    <div class={StyleComponent.card.price.number}>{number}</div>
</div>
