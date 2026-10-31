<script lang="ts">
  import FormUsage from "./FormUsage.svelte";
  import svelteSource from "./FormUsage.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/form/ui/form.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/form/ui/form.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/form/ui/form.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="form" name="Form" description="Formulário configurável com campos de entrada, selecção e validação.">
  {#snippet preview()}<FormUsage />{/snippet}
  {#snippet svelte()}
    <SourceCode path="src/components/local/ui/editor/terminal/form/FormUsage.svelte" source={svelteSource} />
  {/snippet}
  {#snippet react()}
    <SourceCode path="src/project/ui/molecules/form/ui/form.tsx" source={reactSource} />
  {/snippet}
  {#snippet vue()}
    <SourceCode path="src/project/ui/molecules/form/ui/form.vue" source={vueSource} />
  {/snippet}
  {#snippet flutter()}
    <SourceCode path="src/project/ui/molecules/form/ui/form.dart" source={flutterSource} />
  {/snippet}
</Terminal>
