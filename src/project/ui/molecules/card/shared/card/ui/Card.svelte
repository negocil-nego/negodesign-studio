<script lang="ts">
    import type { CardProps } from "../data/types";

    let { ...restProps }: CardProps = $props();
</script>

<div class="card" id="card">
    <div {...restProps} />
</div>