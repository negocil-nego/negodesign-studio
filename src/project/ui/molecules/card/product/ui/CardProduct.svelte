<script lang="ts">
    import type { CardProductProps } from "../data/types";
    import CardProduct01 from "./card-01/CardProduct01.svelte";
    import CardProduct02 from "./card-02/CardProduct02.svelte";

    let { variant = "01", ...restProps }: CardProductProps = $props();
</script>

{#if variant === "01"}
    <CardProduct01 {...restProps} />
{:else if variant === "02"}
    <CardProduct02 {...restProps} />
{/if}
