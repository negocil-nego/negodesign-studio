<script lang="ts">
    import { StyleComponent } from "../../../../../../data/style.component";
    import type { CardPhoneProps } from "../data/types";

    let { phone }: CardPhoneProps = $props();
</script>
<div class={StyleComponent.card.phone.root} id="card-phone">
    <div class={StyleComponent.card.phone.number}>{phone}</div>
</div>
