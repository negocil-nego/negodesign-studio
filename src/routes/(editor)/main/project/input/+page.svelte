<script lang="ts">
  import EditorPageShell from "../../../../../components/local/ui/editor/EditorPageShell.svelte";
  import AtomicComponentScreen from "../../../../../components/local/ui/editor/screen/AtomicComponentScreen.svelte";
  import { componentCatalog } from "../../../../../components/local/data/components";
  import { activeProject } from "../../../../../components/local/data/projects";
  import { copyComponentToActiveProject } from "../../../../../components/local/data/component-copy";

  const inputComponents = componentCatalog.filter(
    (c) =>
      (c.group === "Atom · Input" ||
        (c.group === "Atom" && c.category === "Input")) &&
      c.id !== "input"
  );
  
  let copyingId = $state<string | null>(null);

  async function handleCopy(component: any) {
    if (!component.dependence) return;
    copyingId = component.id;
    try {
      await copyComponentToActiveProject(component.id);
    } catch (error) {
      console.error(error);
    } finally {
      copyingId = null;
    }
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
  
  <AtomicComponentScreen 
    components={inputComponents} 
    viewMode="terminal" 
    {copyingId} 
    onCopy={handleCopy} 
  />
</EditorPageShell>
