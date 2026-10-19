<script lang="ts">
  import CircleButton from "./CircleButton.svelte";
  import { IconNamed } from "../../../../hooks/icon.svelte";

  type Props = {
    onclick?: () => void;
  };

  let { onclick }: Props = $props();
</script>

<CircleButton icon={IconNamed.FolderAdd} title="Novo Projecto" {onclick} />
