<script lang="ts">
    import type { CardProfileProps } from "../data/types";
    import CardProfile01 from "./card-01/CardProfile01.svelte";
    import CardProfile02 from "./card-02/CardProfile02.svelte";

    let { variant = "01", ...restProps }: CardProfileProps = $props();
</script>

{#if variant === "01"}
    <CardProfile01 {...restProps} />
{:else if variant === "02"}
    <CardProfile02 {...restProps} />
{/if}
