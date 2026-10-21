<script lang="ts">
  import InputText from "../../../../../../project/ui/atoms/input/children/text/ui/InputText.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/input/children/text/ui/InputText.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/input/children/text/ui/InputText.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/input/children/text/ui/InputText.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/input/children/text/ui/InputText.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal>
  {#snippet preview()}
    <InputText
      labelProps={{ label: "Texto", for: "docs-input-text" }}
      inputProps={{
        id: "docs-input-text",
        type: "text",
        placeholder: "Introduza um valor",
        required: true,
      }}
    />
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/atoms/input/children/text/ui/InputText.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/atoms/input/children/text/ui/InputText.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/atoms/input/children/text/ui/InputText.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/atoms/input/children/text/ui/InputText.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
