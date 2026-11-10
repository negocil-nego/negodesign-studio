<script lang="ts">
  import Indicators from "../../../../../../project/ui/molecules/indicators/ui/Indicators.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/indicators/ui/Indicators.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/indicators/ui/Indicators.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/indicators/ui/Indicators.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/indicators/ui/Indicators.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="indicators">
  {#snippet preview()}
    <div class="w-full max-w-2xl">
      <Indicators />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode path="src/project/ui/molecules/indicators/ui/Indicators.svelte" source={svelteSource} />
  {/snippet}

  {#snippet react()}
    <SourceCode path="src/project/ui/molecules/indicators/ui/Indicators.tsx" source={reactSource} />
  {/snippet}

  {#snippet vue()}
    <SourceCode path="src/project/ui/molecules/indicators/ui/Indicators.vue" source={vueSource} />
  {/snippet}

  {#snippet flutter()}
    <SourceCode path="src/project/ui/molecules/indicators/ui/Indicators.dart" source={flutterSource} />
  {/snippet}
</Terminal>
