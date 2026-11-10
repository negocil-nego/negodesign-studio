<script lang="ts">
  import InputRange from "../../../../../../project/ui/atoms/input/children/range/ui/InputRange.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/input/children/range/ui/InputRange.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/input/children/range/ui/InputRange.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/input/children/range/ui/InputRange.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/input/children/range/ui/InputRange.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="input-range">
  {#snippet preview()}
    <div class="w-full max-w-md space-y-4">
      <InputRange id="docs-range-1" label="Example range" value={50} />
      <InputRange id="docs-range-2" label="Disabled range" value={35} disabled={true} />
      <InputRange id="docs-range-3" label="Step range (0..5)" value={3} min={0} max={5} step={1} />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode path="src/project/ui/atoms/input/children/range/ui/InputRange.svelte" source={svelteSource} />
  {/snippet}

  {#snippet react()}
    <SourceCode path="src/project/ui/atoms/input/children/range/ui/InputRange.tsx" source={reactSource} />
  {/snippet}

  {#snippet vue()}
    <SourceCode path="src/project/ui/atoms/input/children/range/ui/InputRange.vue" source={vueSource} />
  {/snippet}

  {#snippet flutter()}
    <SourceCode path="src/project/ui/atoms/input/children/range/ui/InputRange.dart" source={flutterSource} />
  {/snippet}
</Terminal>
