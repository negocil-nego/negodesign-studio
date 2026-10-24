<script lang="ts">
    import { StyleComponent } from "../../../../../../data/style.component";
    import type { CardProps } from "../data/types";

    let { ...restProps }: CardProps = $props();
</script>

<div class={StyleComponent.card.root} id="card">
    <div {...restProps} />
</div>