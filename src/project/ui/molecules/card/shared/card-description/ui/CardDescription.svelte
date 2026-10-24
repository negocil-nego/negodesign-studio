<script lang="ts">
    import { StyleComponent } from "../../../../../../data/style.component";
    import type { CardDescriptionProps } from "../data/types";

    let { description, className }: CardDescriptionProps = $props();
</script>
<div class={`card-description ${className ?? ""}`} id="card-description">
    <div class={StyleComponent.card.description}>
        {description}
    </div>
</div>
