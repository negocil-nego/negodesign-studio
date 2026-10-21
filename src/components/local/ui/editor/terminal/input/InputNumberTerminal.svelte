<script lang="ts">
  import InputNumber from "../../../../../../project/ui/atoms/input/children/number/ui/InputNumber.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/input/children/number/ui/InputNumber.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/input/children/number/ui/InputNumber.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/input/children/number/ui/InputNumber.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/input/children/number/ui/InputNumber.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="input-number">
  {#snippet preview()}
    <InputNumber
      labelProps={{ label: "Número", for: "docs-input-number" }}
      inputProps={{
        id: "docs-input-number",
        type: "number",
        placeholder: "0",
      }}
    />
  {/snippet}

  {#snippet svelte()}
    <SourceCode path="src/project/ui/atoms/input/children/number/ui/InputNumber.svelte" source={svelteSource} />
  {/snippet}

  {#snippet react()}
    <SourceCode path="src/project/ui/atoms/input/children/number/ui/InputNumber.tsx" source={reactSource} />
  {/snippet}

  {#snippet vue()}
    <SourceCode path="src/project/ui/atoms/input/children/number/ui/InputNumber.vue" source={vueSource} />
  {/snippet}

  {#snippet flutter()}
    <SourceCode path="src/project/ui/atoms/input/children/number/ui/InputNumber.dart" source={flutterSource} />
  {/snippet}
</Terminal>
