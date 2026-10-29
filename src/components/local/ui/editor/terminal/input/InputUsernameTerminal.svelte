<script lang="ts">
  import InputUsername from "../../../../../../project/ui/atoms/input/children/username/ui/InputUsername.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/input/children/username/ui/InputUsername.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/input/children/username/ui/InputUsername.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/input/children/username/ui/InputUsername.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/input/children/username/ui/InputUsername.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="input-username">
  {#snippet preview()}
    <InputUsername
      labelProps={{ label: "Identificação", for: "docs-input-username" }}
      inputProps={{
        id: "docs-input-username",
        name: "username",
        type: "text",
        placeholder: "Nome, email ou +244 900 000 000",
        required: true,
      }}
    />
  {/snippet}

  {#snippet svelte()}
    <SourceCode path="src/project/ui/atoms/input/children/username/ui/InputUsername.svelte" source={svelteSource} />
  {/snippet}

  {#snippet react()}
    <SourceCode path="src/project/ui/atoms/input/children/username/ui/InputUsername.tsx" source={reactSource} />
  {/snippet}

  {#snippet vue()}
    <SourceCode path="src/project/ui/atoms/input/children/username/ui/InputUsername.vue" source={vueSource} />
  {/snippet}

  {#snippet flutter()}
    <SourceCode path="src/project/ui/atoms/input/children/username/ui/InputUsername.dart" source={flutterSource} />
  {/snippet}
</Terminal>
