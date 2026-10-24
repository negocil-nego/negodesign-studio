<script lang="ts">
    import type { SkeletonProps } from "../data/types";
    import { StyleComponent } from "../../../../data/style.component";
    let { className = "" }: SkeletonProps = $props();
</script>

<div
    aria-hidden="true"
    class={`${StyleComponent.skeleton.default} ${className}`}
></div>