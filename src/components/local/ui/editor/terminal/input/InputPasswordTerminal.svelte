<script lang="ts">
  import InputPassword from "../../../../../../project/ui/atoms/input/children/password/ui/InputPassword.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/input/children/password/ui/InputPassword.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/input/children/password/ui/InputPassword.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/input/children/password/ui/InputPassword.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/input/children/password/ui/InputPassword.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="input-password">
  {#snippet preview()}
    <InputPassword
      labelProps={{ label: "Palavra-passe", for: "docs-input-password" }}
      inputProps={{
        id: "docs-input-password",
        type: "password",
        placeholder: "Introduza a palavra-passe",
      }}
    />
  {/snippet}

  {#snippet svelte()}
    <SourceCode path="src/project/ui/atoms/input/children/password/ui/InputPassword.svelte" source={svelteSource} />
  {/snippet}

  {#snippet react()}
    <SourceCode path="src/project/ui/atoms/input/children/password/ui/InputPassword.tsx" source={reactSource} />
  {/snippet}

  {#snippet vue()}
    <SourceCode path="src/project/ui/atoms/input/children/password/ui/InputPassword.vue" source={vueSource} />
  {/snippet}

  {#snippet flutter()}
    <SourceCode path="src/project/ui/atoms/input/children/password/ui/InputPassword.dart" source={flutterSource} />
  {/snippet}
</Terminal>
