<script lang="ts">
  import InputUsernameOrEmail from "../../../../../../project/ui/atoms/input/children/username/varient/username-or-email/ui/InputUsernameOrEmail.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/input/children/username/varient/username-or-email/ui/InputUsernameOrEmail.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/input/children/username/varient/username-or-email/ui/InputUsernameOrEmail.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/input/children/username/varient/username-or-email/ui/InputUsernameOrEmail.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/input/children/username/varient/username-or-email/ui/InputUsernameOrEmail.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="input-username-or-email">
  {#snippet preview()}
    <InputUsernameOrEmail labelProps={{ label: "Utilizador ou email", for: "docs-username-email" }} inputProps={{ id: "docs-username-email", type: "text", placeholder: "utilizador ou email@exemplo.com" }} />
  {/snippet}
  {#snippet svelte()}
    <SourceCode path="src/project/ui/atoms/input/children/username/varient/username-or-email/ui/InputUsernameOrEmail.svelte" source={svelteSource} />
  {/snippet}
  {#snippet react()}
    <SourceCode path="src/project/ui/atoms/input/children/username/varient/username-or-email/ui/InputUsernameOrEmail.tsx" source={reactSource} />
  {/snippet}
  {#snippet vue()}
    <SourceCode path="src/project/ui/atoms/input/children/username/varient/username-or-email/ui/InputUsernameOrEmail.vue" source={vueSource} />
  {/snippet}
  {#snippet flutter()}
    <SourceCode path="src/project/ui/atoms/input/children/username/varient/username-or-email/ui/InputUsernameOrEmail.dart" source={flutterSource} />
  {/snippet}
</Terminal>
