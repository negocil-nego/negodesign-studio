<script lang="ts">
    import ButtonBase from "../../../shared/base/ui/ButtonBase.svelte";
    import SolidButton from "../../solid/ui/SolidButton.svelte";
    import OutlineButton from "../../outline/ui/OutlineButton.svelte";
    import type { ButtonProps } from "../type/types";
    let {
        variant,
        ...restProps
    }: ButtonProps = $props();
</script>

{#if variant === "solid"}
    <SolidButton {...restProps} />
{:else if variant === "outline"}
    <OutlineButton {...restProps} />
{:else}
    <ButtonBase {...restProps} />
{/if}


