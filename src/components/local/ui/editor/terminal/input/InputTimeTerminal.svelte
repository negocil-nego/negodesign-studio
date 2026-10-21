<script lang="ts">
  import InputTime from "../../../../../../project/ui/atoms/input/children/time/ui/InputTime.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/input/children/time/ui/InputTime.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/input/children/time/ui/InputTime.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/input/children/time/ui/InputTime.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/input/children/time/ui/InputTime.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="input-time">
  {#snippet preview()}
    <InputTime
      labelProps={{ label: "Hora", for: "docs-input-time" }}
      inputProps={{
        id: "docs-input-time",
        type: "time",
        placeholder: "",
      }}
    />
  {/snippet}

  {#snippet svelte()}
    <SourceCode path="src/project/ui/atoms/input/children/time/ui/InputTime.svelte" source={svelteSource} />
  {/snippet}

  {#snippet react()}
    <SourceCode path="src/project/ui/atoms/input/children/time/ui/InputTime.tsx" source={reactSource} />
  {/snippet}

  {#snippet vue()}
    <SourceCode path="src/project/ui/atoms/input/children/time/ui/InputTime.vue" source={vueSource} />
  {/snippet}

  {#snippet flutter()}
    <SourceCode path="src/project/ui/atoms/input/children/time/ui/InputTime.dart" source={flutterSource} />
  {/snippet}
</Terminal>
