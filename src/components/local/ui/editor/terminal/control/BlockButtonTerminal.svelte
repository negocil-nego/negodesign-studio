<script lang="ts">
  import BlockButton from "../../../../../../project/ui/atoms/button/variant/block/ui/BlockButton.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/button/variant/block/ui/BlockButton.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/button/variant/block/ui/BlockButton.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/button/variant/block/ui/BlockButton.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/button/variant/block/ui/BlockButton.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="block-button">
  {#snippet preview()}
    <div class="d-grid w-full max-w-md gap-2">
      <BlockButton label="Block button" />
      <BlockButton label="Block button" />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode path="src/project/ui/atoms/button/variant/block/ui/BlockButton.svelte" source={svelteSource} />
  {/snippet}

  {#snippet react()}
    <SourceCode path="src/project/ui/atoms/button/variant/block/ui/BlockButton.tsx" source={reactSource} />
  {/snippet}

  {#snippet vue()}
    <SourceCode path="src/project/ui/atoms/button/variant/block/ui/BlockButton.vue" source={vueSource} />
  {/snippet}

  {#snippet flutter()}
    <SourceCode path="src/project/ui/atoms/button/variant/block/ui/BlockButton.dart" source={flutterSource} />
  {/snippet}
</Terminal>
