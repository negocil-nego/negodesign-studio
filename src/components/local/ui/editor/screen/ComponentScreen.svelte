<script lang="ts">
  import { componentCatalog, type ComponentDefinition } from "../../../data/components";
  import { activeProject } from "../../../data/projects";
  import { componentScreenNavigation } from "../shared/component-screen-navigation";
  import InputScreen from "./InputScreen.svelte";
  import CardScreen from "./CardScreen.svelte";
  import MenuScreen from "./MenuScreen.svelte";
  import ComponentScreenHeader from "../shared/ComponentScreenHeader.svelte";
  import ComponentScreenGroup from "../shared/ComponentScreenGroup.svelte";
  import ComponentCopyDialog from "../shared/ComponentCopyDialog.svelte";

  const catalogById = Object.fromEntries(componentCatalog.map((component) => [component.id, component]));
  const groups = componentScreenNavigation.map((group) => ({
    ...group,
    components: group.ids.map((id) => catalogById[id]).filter((item): item is ComponentDefinition => Boolean(item)),
  }));
  let componentToCopy = $state<ComponentDefinition | null>(null);
  let status = $state("");

  function requestCopy(component: ComponentDefinition) {
    if (!component.dependence) return;
    if (!$activeProject) {
      status = "Seleccione primeiro um projecto activo no Inspector.";
      return;
    }
    status = "";
    componentToCopy = component;
  }
</script>

<main class="w-full min-w-0 space-y-8">
  <ComponentScreenHeader count={componentCatalog.length} {status} />
  {#each groups as group (group.label)}
    {#if group.components.length}
      <ComponentScreenGroup label={group.label} badge={group.badge} description={group.description} count={group.components.length}>
        {#if group.type === "input"}
          <InputScreen components={group.components} onCopy={requestCopy} />
        {:else if group.type === "card"}
          <CardScreen components={group.components} onCopy={requestCopy} />
        {:else}
          <MenuScreen showHeader={false} />
        {/if}
      </ComponentScreenGroup>
    {/if}
  {/each}
</main>

{#if componentToCopy}
  <ComponentCopyDialog
    component={componentToCopy}
    onclose={() => (componentToCopy = null)}
    onmessage={(message) => (status = message)}
  />
{/if}
