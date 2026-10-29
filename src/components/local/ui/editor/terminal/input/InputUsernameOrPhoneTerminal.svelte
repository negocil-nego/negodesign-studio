<script lang="ts">
  import InputUsernameOrPhone from "../../../../../../project/ui/atoms/input/children/username/varient/username-or-phone/ui/InputUsernameOrPhone.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/input/children/username/varient/username-or-phone/ui/InputUsernameOrPhone.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/input/children/username/varient/username-or-phone/ui/InputUsernameOrPhone.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/input/children/username/varient/username-or-phone/ui/InputUsernameOrPhone.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/input/children/username/varient/username-or-phone/ui/InputUsernameOrPhone.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="input-username-or-phone">
  {#snippet preview()}
    <InputUsernameOrPhone labelProps={{ label: "Utilizador ou telefone", for: "docs-username-phone" }} inputProps={{ id: "docs-username-phone", type: "text", placeholder: "utilizador ou +244 900 000 000" }} />
  {/snippet}
  {#snippet svelte()}
    <SourceCode path="src/project/ui/atoms/input/children/username/varient/username-or-phone/ui/InputUsernameOrPhone.svelte" source={svelteSource} />
  {/snippet}
  {#snippet react()}
    <SourceCode path="src/project/ui/atoms/input/children/username/varient/username-or-phone/ui/InputUsernameOrPhone.tsx" source={reactSource} />
  {/snippet}
  {#snippet vue()}
    <SourceCode path="src/project/ui/atoms/input/children/username/varient/username-or-phone/ui/InputUsernameOrPhone.vue" source={vueSource} />
  {/snippet}
  {#snippet flutter()}
    <SourceCode path="src/project/ui/atoms/input/children/username/varient/username-or-phone/ui/InputUsernameOrPhone.dart" source={flutterSource} />
  {/snippet}
</Terminal>
