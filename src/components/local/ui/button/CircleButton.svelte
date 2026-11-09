<script lang="ts">
  import { type IconNamed, getIconClass } from "../../../../hooks/icon.svelte";
  import { LocalStyleComponent } from "../../data/style.component";

  type Props = {
    icon: IconNamed;
    onclick?: () => void;
    title?: string;
  };

  let { icon, onclick, title = "" }: Props = $props();
</script>

<button
  type="button"
  {onclick}
  {title}
  class="{LocalStyleComponent.layout.iconButton} active:scale-95 shadow-sm"
>
  <i
    class={`${getIconClass(icon)} w-5 h-5 text-lg opacity-80 hover:opacity-100 transition-opacity`}
    aria-label={title}
  ></i>
</button>
