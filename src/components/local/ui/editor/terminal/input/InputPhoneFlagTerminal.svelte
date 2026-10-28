<script lang="ts">
  import InputPhoneFlag from "../../../../../../project/ui/atoms/input/children/phone/varient/flag/ui/InputPhoneFlag.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/input/children/phone/varient/flag/ui/InputPhoneFlag.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/input/children/phone/varient/flag/ui/InputPhoneFlag.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/input/children/phone/varient/flag/ui/InputPhoneFlag.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/input/children/phone/varient/flag/ui/InputPhoneFlag.dart?raw";
  import { FlagNamed } from "../../../../../../hooks/flag.svelte";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="input-phone-flag">
  {#snippet preview()}
    <InputPhoneFlag
      labelProps={{ label: "Telefone", for: "docs-input-phone-flag" }}
      flag={[FlagNamed.Ao, FlagNamed.Ng]}
      locale="pt"
      inputProps={{
        id: "docs-input-phone-flag",
        type: "tel",
        placeholder: "900 000 000",
      }}
    />
  {/snippet}

  {#snippet svelte()}
    <SourceCode path="src/project/ui/atoms/input/children/phone/varient/flag/ui/InputPhoneFlag.svelte" source={svelteSource} />
  {/snippet}

  {#snippet react()}
    <SourceCode path="src/project/ui/atoms/input/children/phone/varient/flag/ui/InputPhoneFlag.tsx" source={reactSource} />
  {/snippet}

  {#snippet vue()}
    <SourceCode path="src/project/ui/atoms/input/children/phone/varient/flag/ui/InputPhoneFlag.vue" source={vueSource} />
  {/snippet}

  {#snippet flutter()}
    <SourceCode path="src/project/ui/atoms/input/children/phone/varient/flag/ui/InputPhoneFlag.dart" source={flutterSource} />
  {/snippet}
</Terminal>
