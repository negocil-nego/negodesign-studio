<script lang="ts">
  import InputPhone from "../../../../../../project/ui/atoms/input/children/phone/ui/InputPhone.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/input/children/phone/ui/InputPhone.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/input/children/phone/ui/InputPhone.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/input/children/phone/ui/InputPhone.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/input/children/phone/ui/InputPhone.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="input-phone">
  {#snippet preview()}
    <InputPhone
      labelProps={{ label: "Telefone", for: "docs-input-phone" }}
      inputProps={{
        id: "docs-input-phone",
        type: "tel",
        placeholder: "+244 900 000 000",
      }}
    />
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/atoms/input/children/phone/ui/InputPhone.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/atoms/input/children/phone/ui/InputPhone.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/atoms/input/children/phone/ui/InputPhone.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/atoms/input/children/phone/ui/InputPhone.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>

