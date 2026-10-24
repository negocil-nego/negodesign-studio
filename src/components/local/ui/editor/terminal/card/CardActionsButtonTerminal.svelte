<script lang="ts">
  import CardActionsButton from "../../../../../../project/ui/molecules/card/shared/card-actions-button/ui/CardActionsButton.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/card/shared/card-actions-button/ui/CardActionsButton.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/card/shared/card-actions-button/ui/CardActionsButton.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/card/shared/card-actions-button/ui/CardActionsButton.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/card/shared/card-actions-button/ui/CardActionsButton.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";

  import type { CardActionButton } from "../../../../../../project/ui/molecules/card/shared/card-actions-button/data/types";

  const actions: CardActionButton[] = [
    { icon: "Link", label: "Partilhar" },
    { icon: "MoreHorizontal", label: "Mais" },
  ];
</script>

<Terminal id="card-actions-button">
  {#snippet preview()}
    <CardActionsButton {actions} />
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-actions-button/ui/CardActionsButton.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-actions-button/ui/CardActionsButton.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-actions-button/ui/CardActionsButton.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-actions-button/ui/CardActionsButton.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
