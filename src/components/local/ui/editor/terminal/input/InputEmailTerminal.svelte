<script lang="ts">
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
  import InputEmail from "../../../../../../project/ui/atoms/input/children/email/ui/InputEmail.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/input/children/email/ui/InputEmail.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/input/children/email/ui/InputEmail.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/input/children/email/ui/InputEmail.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/input/children/email/ui/InputEmail.dart?raw";
</script>

<Terminal>
  {#snippet preview()}
    <InputEmail
      labelProps={{ label: "Email", for: "docs-input-email" }}
      inputProps={{
        id: "docs-input-email",
        type: "email",
        placeholder: "nome@exemplo.com",
        required: true,
      }}
    />
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/atoms/input/children/email/ui/InputEmail.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/atoms/input/children/email/ui/InputEmail.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/atoms/input/children/email/ui/InputEmail.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/atoms/input/children/email/ui/InputEmail.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
