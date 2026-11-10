<script lang="ts">
  import EditorPageShell from "../../../../../components/local/ui/editor/EditorPageShell.svelte";
  import InputScreen from "../../../../../components/local/ui/editor/screen/InputScreen.svelte";
  import ComponentCopyDialog from "../../../../../components/local/ui/editor/shared/ComponentCopyDialog.svelte";
  import { componentCatalog, type ComponentDefinition } from "../../../../../components/local/data/components";

  const inputComponents = componentCatalog.filter(
    (c) =>
      (c.group === "Atom · Input" ||
        (c.group === "Atom" && c.category === "Input")) &&
      c.id !== "input",
  );

  let componentToCopy = $state<ComponentDefinition | null>(null);

  function handleCopy(component: ComponentDefinition) {
    if (!component.dependence) return;
    componentToCopy = component;
  }
</script>

<EditorPageShell>
  <div>
    <p class="text-[10px] font-semibold uppercase tracking-[0.18em] text-blue-400">
      Átomos / Input
    </p>
    <h2 class="mt-1 text-lg font-semibold text-zinc-100">Campos de entrada</h2>
    <p class="mt-2 mb-6 text-xs leading-5 text-zinc-400">
      Pré-visualização interactiva e código-fonte dos componentes de entrada.
    </p>
  </div>

  <InputScreen
    components={inputComponents}
    viewMode="terminal"
    copyingId={null}
    onCopy={handleCopy}
  />
</EditorPageShell>

{#if componentToCopy}
  <ComponentCopyDialog
    component={componentToCopy}
    onclose={() => (componentToCopy = null)}
    onmessage={() => {}}
  />
{/if}
