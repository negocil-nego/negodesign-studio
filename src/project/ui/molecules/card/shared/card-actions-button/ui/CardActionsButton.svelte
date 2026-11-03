<script lang="ts">
    import OutlineButton from "../../../../../atoms/button/variant/outline/ui/OutlineButton.svelte";
    import Icon from "../../../../../atoms/icon/ui/Icon.svelte";
    import type { CardActionsButtonProps } from "../data/types";
    import { StyleComponent } from "../../../../../../data/style.component";

    let { actions, variant = "default" }: CardActionsButtonProps = $props();

    const s = StyleComponent.card.cardActionsButton;
</script>

<div class={s.root}>
    {#each actions as action, i}
        {#if variant === "default"}
            <!-- Original: outline button with optional icon + label -->
            <OutlineButton
                onClick={action.onClick}
                className={s.button}
                text={action.label ?? ""}
                icon={action.icon ? { name: action.icon, className: "text-lg" } : undefined}
            />

        {:else if variant === "icon-outline"}
            <!-- Icon-only round outline button (e.g. Edit Profile) -->
            <button onclick={action.onClick} class={s.iconOutline}>
                {#if action.icon}
                    <Icon name={action.icon} className={s.iconOutlineIcon} />
                {/if}
                {#if action.label}
                    <span class={s.iconOutlineLabel}>{action.label}</span>
                {/if}
            </button>

        {:else if variant === "mixed"}
            {#if i === 0}
                <!-- First action: solid pill button (Follow) -->
                <button onclick={action.onClick} class={s.mixedSolid}>
                    {#if action.icon}
                        <Icon name={action.icon} className="text-sm" />
                    {/if}
                    {#if action.label}
                        <span>{action.label}</span>
                    {/if}
                </button>
            {:else}
                <!-- Remaining actions: icon-only round outline (Message) -->
                <button onclick={action.onClick} class={s.mixedIcon} title={action.label ?? ""}>
                    {#if action.icon}
                        <Icon name={action.icon} className={s.mixedIconIcon} />
                    {/if}
                </button>
            {/if}
        {/if}
    {/each}
</div>
