<script lang="ts">
    import Icon from "../../../../icon/ui/Icon.svelte";
    import type { ButtonBaseProps } from "../data/types";
    
    let {
        icon,
        text,
        isLoading,
        className,
        isDisabled = false,
        onClick,
    }: ButtonBaseProps = $props();
</script>

<button
    class={className}
    onclick={onClick}
    disabled={isDisabled || isLoading}
>
    {#if icon}
        <Icon {...icon} />
    {/if}
    {#if text}
        <span>{text}</span>
    {/if}
</button>
