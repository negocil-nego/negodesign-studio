<script lang="ts">
  import InputAge from "../../../../../../project/ui/atoms/input/children/age/ui/InputAge.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/input/children/age/ui/InputAge.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/input/children/age/ui/InputAge.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/input/children/age/ui/InputAge.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/input/children/age/ui/InputAge.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="input-age">
  {#snippet preview()}
    <InputAge
      labelProps={{ label: "Idade", for: "docs-input-age" }}
      inputProps={{
        id: "docs-input-age",
        type: "number",
        placeholder: "Introduza a idade",
      }}
    />
  {/snippet}

  {#snippet svelte()}
    <SourceCode path="src/project/ui/atoms/input/children/age/ui/InputAge.svelte" source={svelteSource} />
  {/snippet}

  {#snippet react()}
    <SourceCode path="src/project/ui/atoms/input/children/age/ui/InputAge.tsx" source={reactSource} />
  {/snippet}

  {#snippet vue()}
    <SourceCode path="src/project/ui/atoms/input/children/age/ui/InputAge.vue" source={vueSource} />
  {/snippet}

  {#snippet flutter()}
    <SourceCode path="src/project/ui/atoms/input/children/age/ui/InputAge.dart" source={flutterSource} />
  {/snippet}
</Terminal>
