<script lang="ts">
    import type { CardTitleProps } from "../data/types";

    let { title }: CardTitleProps = $props();
</script>
<div class="card-title" id="card-title">
    <div class="card-title-text">{title}</div>
</div>
