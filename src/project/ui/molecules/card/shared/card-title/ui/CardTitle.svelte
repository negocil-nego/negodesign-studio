<script lang="ts">
    import { StyleComponent } from "../../../../../../data/style.component";
    import type { CardTitleProps } from "../data/types";

    let { title, className }: CardTitleProps = $props();
</script>
<div class={`${StyleComponent.card.title.root} ${className ?? ""}`} id="card-title">
    <div class={StyleComponent.card.title.text}>{title}</div>
</div>
