<script lang="ts">
  import Card from "../../../../../../project/ui/molecules/card/shared/card/ui/Card.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/card/shared/card/ui/Card.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/card/shared/card/ui/Card.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/card/shared/card/ui/Card.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/card/shared/card/ui/Card.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="card">
  {#snippet preview()}
    <div class="w-full max-w-xs">
      <Card />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card/ui/Card.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card/ui/Card.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card/ui/Card.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card/ui/Card.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
