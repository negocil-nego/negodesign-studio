<script lang="ts">
    import type { CardPriceProps } from "../data/types";

    let { number }: CardPriceProps = $props();
</script>
<div class="card-price" id="card-price">
    <div class="card-price-number">{number}</div>
</div>
