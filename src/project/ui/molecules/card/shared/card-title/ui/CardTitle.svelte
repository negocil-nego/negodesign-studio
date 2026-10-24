<script lang="ts">
    import type { CardTitleProps } from "../data/types";

    let { title, className }: CardTitleProps = $props();
</script>
<div class={`card-title ${className ?? ""}`} id="card-title">
    <div class="card-title-text">{title}</div>
</div>
