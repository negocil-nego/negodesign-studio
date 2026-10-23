<script lang="ts">
    import { StyleComponent } from "../../../../../data/style.component";
    import ButtonBase from "../../shared/base/ui/ButtonBase.svelte";
    import type { SolidButtonProps } from "../data/types";

    let { ...restProps }: SolidButtonProps = $props();
</script>

<ButtonBase className={StyleComponent.button.solid.default} {...restProps} />
