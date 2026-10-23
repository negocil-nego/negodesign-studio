<script lang="ts">
    import type { CardDescriptionProps } from "../data/types";

    let { description }: CardDescriptionProps = $props();
</script>
<div class="card-description" id="card-description">
    <div class="card-description-text line-clamp-2 xl:line-clamp-3 2xl:line-clamp-4">
        {description}
    </div>
</div>
