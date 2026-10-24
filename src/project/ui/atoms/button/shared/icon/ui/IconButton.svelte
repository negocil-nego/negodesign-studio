<script lang="ts">;
    import { StyleComponent } from "../../../../../../data/style.component";
    import Button from "../../../variant/default/ui/Button.svelte";
    import type { IconButtonProps } from "../data/types";

    let { icon, isLoading = false, className, onClick }: IconButtonProps = $props();
</script>
<Button className={StyleComponent.iconCenterButton.default} {onClick} isSkeleton={isLoading} {icon} />
