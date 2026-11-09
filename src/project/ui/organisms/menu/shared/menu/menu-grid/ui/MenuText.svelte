<script lang="ts">
  import { LocalStyleComponent } from "../../../../../../../../components/local/data/style.component";
  import Icon from "../../../../../../atoms/icon/ui/Icon.svelte";
  import Text from "../../../../../../atoms/text/ui/Text.svelte";
  import type { MenuTextProps } from "../../menu-text/data/types";

  let { icon, textProps }: MenuTextProps = $props();
</script>

<div class={LocalStyleComponent.menu.text.root}>
  {#if icon}
    <Icon
      name={icon.name}
      isLoading={icon.isLoading}
      className={`${LocalStyleComponent.menu.text.icon} ${icon.className ?? ""}`}
    />
  {/if}
  {#if textProps}
    <div class={LocalStyleComponent.menu.text.label}>
      <Text {...textProps} />
    </div>
  {/if}
</div>
