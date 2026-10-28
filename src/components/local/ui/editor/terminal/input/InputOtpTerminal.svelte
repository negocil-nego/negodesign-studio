<script lang="ts">
  import InputOtp from "../../../../../../project/ui/atoms/input/children/otp/ui/InputOtp.svelte";
  import svelteSource from "../../../../../../project/ui/atoms/input/children/otp/ui/InputOtp.svelte?raw";
  import reactSource from "../../../../../../project/ui/atoms/input/children/otp/ui/InputOtp.tsx?raw";
  import vueSource from "../../../../../../project/ui/atoms/input/children/otp/ui/InputOtp.vue?raw";
  import flutterSource from "../../../../../../project/ui/atoms/input/children/otp/ui/InputOtp.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="input-otp">
  {#snippet preview()}
    <InputOtp id="docs-input-otp" name="verification-code" value="456789" />
  {/snippet}

  {#snippet svelte()}
    <SourceCode path="src/project/ui/atoms/input/children/otp/ui/InputOtp.svelte" source={svelteSource} />
  {/snippet}

  {#snippet react()}
    <SourceCode path="src/project/ui/atoms/input/children/otp/ui/InputOtp.tsx" source={reactSource} />
  {/snippet}

  {#snippet vue()}
    <SourceCode path="src/project/ui/atoms/input/children/otp/ui/InputOtp.vue" source={vueSource} />
  {/snippet}

  {#snippet flutter()}
    <SourceCode path="src/project/ui/atoms/input/children/otp/ui/InputOtp.dart" source={flutterSource} />
  {/snippet}
</Terminal>
