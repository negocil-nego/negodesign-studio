<script lang="ts">
  import { type IconNamed, getIconUrl } from "../../../../hooks/icon.svelte";

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
  class="blender-icon-btn active:scale-95 shadow-sm"
>
  <img src={getIconUrl(icon)} alt={title} class="w-4 h-4 brightness-0 invert opacity-80 hover:opacity-100 transition-opacity" />
</button>
