<script lang="ts">
    import { StyleComponent } from "../../../../../data/style.component";
    import ButtonBase from "../../shared/base/ui/ButtonBase.svelte";
    import type { OutlineButtonProps } from "../data/types";

    let { ...restProps }: OutlineButtonProps = $props();
</script>

<ButtonBase className={StyleComponent.button.outline.default} {...restProps} />
