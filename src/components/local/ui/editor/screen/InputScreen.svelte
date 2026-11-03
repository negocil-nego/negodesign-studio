<script lang="ts">
  import { componentCatalog, type ComponentDefinition } from "../../../data/components";
  import AtomicComponentScreen from "./AtomicComponentScreen.svelte";

  let {
    components,
    copyingId = null,
    onCopy = () => {},
    viewMode = "gallery",
  }: {
    components?: ComponentDefinition[];
    copyingId?: string | null;
    onCopy?: (component: ComponentDefinition) => void;
    viewMode?: "gallery" | "terminal";
  } = $props();
  const inputComponents = $derived(components ?? componentCatalog.filter(
    (component) =>
      (component.group === "Atom · Input" ||
        (component.group === "Atom" && component.category === "Input")) &&
      component.id !== "input",
  ));
</script>

<AtomicComponentScreen components={inputComponents} {copyingId} {onCopy} {viewMode} />
