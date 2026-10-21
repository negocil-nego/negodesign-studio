<script lang="ts">
  import InputDate from "../../../../../../project/ui/atoms/input/children/date/ui/InputDate.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/input/children/date/ui/InputDate.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/input/children/date/ui/InputDate.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/input/children/date/ui/InputDate.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/input/children/date/ui/InputDate.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="input-date">
  {#snippet preview()}
    <InputDate
      labelProps={{ label: "Data", for: "docs-input-date" }}
      inputProps={{
        id: "docs-input-date",
        type: "date",
        placeholder: "",
      }}
    />
  {/snippet}

  {#snippet svelte()}
    <SourceCode path="src/project/ui/atoms/input/children/date/ui/InputDate.svelte" source={svelteSource} />
  {/snippet}

  {#snippet react()}
    <SourceCode path="src/project/ui/atoms/input/children/date/ui/InputDate.tsx" source={reactSource} />
  {/snippet}

  {#snippet vue()}
    <SourceCode path="src/project/ui/atoms/input/children/date/ui/InputDate.vue" source={vueSource} />
  {/snippet}

  {#snippet flutter()}
    <SourceCode path="src/project/ui/atoms/input/children/date/ui/InputDate.dart" source={flutterSource} />
  {/snippet}
</Terminal>
