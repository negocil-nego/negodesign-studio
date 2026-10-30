<script lang="ts">
  import Terminal from "./Terminal.svelte";
  import SourceCode from "./SourceCode.svelte";
  import Label from "../../../../../project/ui/atoms/label/ui/Label.svelte";
  import svelteSource from "../../../../../project/ui/atoms/label/ui/Label.svelte?raw";
  import reactSource from "../../../../../project/ui/atoms/label/ui/label.tsx?raw";
  import vueSource from "../../../../../project/ui/atoms/label/ui/Label.vue?raw";
  import flutterSource from "../../../../../project/ui/atoms/label/ui/label.dart?raw";
</script>

<Terminal id="label">
  {#snippet preview()}
    <div class="space-y-2">
      <Label label="Nome completo" for="docs-label-name" />
      <input
        id="docs-label-name"
        type="text"
        placeholder="O seu nome"
        class="w-full rounded border border-blender-border bg-blender-base px-3 py-2 text-xs text-zinc-200 placeholder:text-zinc-600 focus:border-blue-500/50 focus:outline-none"
      />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode path="src/project/ui/atoms/label/ui/Label.svelte" source={svelteSource} />
  {/snippet}

  {#snippet react()}
    <SourceCode path="src/project/ui/atoms/label/ui/label.tsx" source={reactSource} />
  {/snippet}

  {#snippet vue()}
    <SourceCode path="src/project/ui/atoms/label/ui/Label.vue" source={vueSource} />
  {/snippet}

  {#snippet flutter()}
    <SourceCode path="src/project/ui/atoms/label/ui/label.dart" source={flutterSource} />
  {/snippet}
</Terminal>

