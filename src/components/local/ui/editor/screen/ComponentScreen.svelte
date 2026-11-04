<script lang="ts">
  import { componentCatalog, type ComponentDefinition } from "../../../data/components";
  import { activeProject } from "../../../data/projects";
  import { componentScreenNavigation } from "../shared/component-screen-navigation";
  import ComponentCopyDialog from "../shared/ComponentCopyDialog.svelte";
  import ComponentScreenGroup from "../shared/ComponentScreenGroup.svelte";
  import LazyComponent from "../shared/LazyComponent.svelte";
  import ComponentScreenHeader from "../shared/ComponentScreenHeader.svelte";
  import Tabs from "../shared/Tabs.svelte";

  type ScreenGroupType = (typeof componentScreenNavigation)[number]["type"];
  type ScreenLoader = () => Promise<{ default: any }>;

  const catalogById = Object.fromEntries(
    componentCatalog.map((component) => [component.id, component]),
  );
  const groups = componentScreenNavigation.map((group) => ({
    ...group,
    components: group.ids
      .map((id) => catalogById[id])
      .filter((item): item is ComponentDefinition => Boolean(item)),
  }));
  const initialGroup = groups[0];
  const tabItems = groups.map((group) => ({
    id: group.type,
    label: group.label,
    count: group.components.length,
  }));

  const screenLoaders: Record<ScreenGroupType, ScreenLoader> = {
    input: () => import("./InputScreen.svelte"),
    card: () => import("./CardScreen.svelte"),
    menu: () => import("./MenuScreen.svelte"),
    sidebar: () => import("./SidebarScreen.svelte"),
    carousel: () => import("./CarouselScreen.svelte"),
    grid: () => import("./GridScreen.svelte"),
    "grid-carousel": () => import("./GridCarouselScreen.svelte"),
  };

  let activeGroupType = $state<ScreenGroupType>(initialGroup?.type ?? "input");
  let activeGroup = $derived(
    groups.find((group) => group.type === activeGroupType) ?? initialGroup,
  );
  let componentToCopy = $state<ComponentDefinition | null>(null);
  let status = $state("");

  function selectGroup(groupType: string) {
    activeGroupType = groupType as ScreenGroupType;
  }

  function requestCopy(component: ComponentDefinition) {
    if (!component.dependence) return;
    if (!$activeProject) {
      status = "Seleccione primeiro um projecto activo no Inspector.";
      return;
    }
    status = "";
    componentToCopy = component;
  }

  const screenProps = $derived.by(() => {
    if (!activeGroup) return {};

    if (activeGroup.type === "input" || activeGroup.type === "card") {
      return {
        components: activeGroup.components,
        onCopy: requestCopy,
      };
    }

    if (
      activeGroup.type === "menu" ||
      activeGroup.type === "carousel" ||
      activeGroup.type === "grid" ||
      activeGroup.type === "grid-carousel"
    ) {
      return { showHeader: false };
    }

    return {};
  });
</script>

<main class="w-full min-w-0 space-y-6">
  <ComponentScreenHeader count={componentCatalog.length} {status} />

  <div class="space-y-4">
    <Tabs
      items={tabItems}
      value={activeGroup?.type ?? "input"}
      panelId="component-screen-panel"
      idPrefix="component-screen-tab"
      onselect={selectGroup}
    />

    {#if activeGroup}
      <div
        id="component-screen-panel"
        role="tabpanel"
        aria-labelledby={`component-screen-tab-${activeGroup.type}`}
        tabindex="0"
      >
        <ComponentScreenGroup
          label={activeGroup.label}
          badge={activeGroup.badge}
          description={activeGroup.description}
          count={activeGroup.components.length}
        >
          {#key activeGroup.type}
            <LazyComponent
              loader={screenLoaders[activeGroup.type]}
              componentProps={screenProps}
            />
          {/key}
        </ComponentScreenGroup>
      </div>
    {/if}
  </div>
</main>

{#if componentToCopy}
  <ComponentCopyDialog
    component={componentToCopy}
    onclose={() => (componentToCopy = null)}
    onmessage={(message) => (status = message)}
  />
{/if}
