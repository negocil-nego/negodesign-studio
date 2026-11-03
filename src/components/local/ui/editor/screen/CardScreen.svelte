<script lang="ts">
  import { componentCatalog, type ComponentDefinition } from "../../../data/components";
  import { copyComponentToActiveProject } from "../../../data/component-copy";
  import MoleculeComponentScreen from "./MoleculeComponentScreen.svelte";

  let internalCopyingId = $state<string | null>(null);
  let {
    components,
    copyingId = null,
    onCopy,
  }: {
    components?: ComponentDefinition[];
    copyingId?: string | null;
    onCopy?: (component: ComponentDefinition) => void;
  } = $props();

  const cardComponents = $derived(components ?? componentCatalog.filter(
    (component) =>
      component.group === "Molecule · Card" ||
      component.group === "Molecule · Card · Shared",
  ));
  const activeCopyingId = $derived(copyingId ?? internalCopyingId);

  async function handleCopy(component: ComponentDefinition) {
    if (onCopy) {
      onCopy(component);
      return;
    }
    internalCopyingId = component.id;
    try {
      await copyComponentToActiveProject(component.id);
    } finally {
      internalCopyingId = null;
    }
  }
</script>

<MoleculeComponentScreen components={cardComponents} copyingId={activeCopyingId} onCopy={handleCopy} />
