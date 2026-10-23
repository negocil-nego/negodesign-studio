<script lang="ts">
    import type { CardMediaProps } from "../data/types";
    import CardMedia01 from "./card-01/CardMedia01.svelte";
    import CardMedia02 from "./card-02/CardMedia02.svelte";

    let { variant = "01", ...restProps }: CardMediaProps = $props();
</script>

{#if variant === "01"}
    <CardMedia01 {...restProps} />
{:else if variant === "02"}
    <CardMedia02 {...restProps} />
{/if}
