<script lang="ts">
  import { StyleComponent } from "../../../../../../data/style.component";
  import ButtonBase from "../../../shared/base/ui/ButtonBase.svelte";
  import type { BlockButtonProps } from "../data/types";

  let { variant = "primary", className = "", ...restProps }: BlockButtonProps = $props();

  const variantClass = $derived(
    variant === "outline"
      ? `${StyleComponent.button.outline.default} btn-lg btn-block`
      : variant === "secondary"
        ? `${StyleComponent.button.default} btn-secondary btn-lg btn-block`
        : StyleComponent.button.block.default,
  );
</script>

<ButtonBase className={`${variantClass} ${className}`.trim()} {...restProps} />
