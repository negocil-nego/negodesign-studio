<script lang="ts">
    import OutlineButton from "../../../../../atoms/button/variant/outline/ui/OutlineButton.svelte";
    import type { CardActionsButtonProps } from "../data/types";
    import { StyleComponent } from "../../../../../../data/style.component";

    let { actions }: CardActionsButtonProps = $props();
</script>

<div class={StyleComponent.card.cardActionsButton.root}>
    {#each actions as action}
        <OutlineButton 
            onClick={action.onClick}
            className={StyleComponent.card.cardActionsButton.button}
            text={action.label}
            icon={action.icon ? { name: action.icon, className: "text-lg" } : undefined}
        />
    {/each}
</div>
