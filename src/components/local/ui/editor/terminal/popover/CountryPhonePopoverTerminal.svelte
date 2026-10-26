<script lang="ts">
  import CountryPhonePopover from "../../../../../../project/ui/molecules/popover/ui/CountryPhonePopover.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/popover/ui/CountryPhonePopover.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/popover/ui/CountryPhonePopover.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/popover/ui/CountryPhonePopover.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/popover/ui/CountryPhonePopover.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
  import { FlagNamed } from "../../../../../../hooks/flag.svelte";
</script>

<Terminal id="country-popover">
  {#snippet preview()}
    <div class="flex items-center justify-center p-8">
      <CountryPhonePopover flag={FlagNamed.Ao} locale="pt" />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/molecules/popover/ui/CountryPhonePopover.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/molecules/popover/ui/CountryPhonePopover.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/molecules/popover/ui/CountryPhonePopover.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/molecules/popover/ui/CountryPhonePopover.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
