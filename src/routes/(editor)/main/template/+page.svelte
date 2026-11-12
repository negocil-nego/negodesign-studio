<script lang="ts">
  import LazyComponent from "../../../../components/local/ui/editor/shared/LazyComponent.svelte";
  import Tabs from "../../../../components/local/ui/editor/shared/Tabs.svelte";
  import {
    templateScreenNavigation,
    type TemplateScreenGroupType,
  } from "../../../../components/local/ui/editor/shared/template-screen-navigation";

  type ScreenLoader = () => Promise<any>;

  const groups = templateScreenNavigation;
  const tabItems = groups.map((group) => ({
    id: group.type,
    label: group.label,
    count: group.count,
  }));
  const totalPages = groups.reduce((total, group) => total + group.count, 0);

  const screenLoaders: Record<TemplateScreenGroupType, ScreenLoader> = {
    hero: () =>
      import(
        "../../../../components/local/ui/editor/screen/HeroTemplateScreen.svelte"
      ),
    authentication: () =>
      import(
        "../../../../components/local/ui/editor/screen/PageAuthenticationScreen.svelte"
      ),
    "product-details": () =>
      import(
        "../../../../components/local/ui/editor/screen/PageProductDetailsScreen.svelte"
      ),
  };

  let activeGroup = $state<TemplateScreenGroupType>("hero");
</script>

<div class="min-h-full w-full bg-blender-base p-4 md:p-6 xl:p-8">
  <div class="mx-auto flex w-full max-w-[1800px] flex-col gap-7">
    <header
      class="flex flex-wrap items-end justify-between gap-4 border-b border-blender-border pb-5"
    >
      <div>
        <p
          class="text-xs font-semibold uppercase tracking-[0.18em] text-blue-500"
        >
          Frontend <span class="px-1 text-zinc-500">/</span> Layouts
        </p>
        <h1
          class="mt-2 text-3xl font-semibold tracking-tight text-zinc-800 dark:text-zinc-100"
        >
          Templates
        </h1>
        <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-400">
          Explore páginas de destaque e fluxos de autenticação prontos para
          adaptar.
        </p>
      </div>
      <span
        class="rounded-full border border-blender-border bg-blender-card px-3 py-1.5 text-xs text-zinc-500 dark:text-zinc-400"
        >{totalPages} páginas disponíveis</span
      >
    </header>

    <div class="space-y-4">
      <Tabs
        items={tabItems}
        value={activeGroup}
        panelId="template-screen-panel"
        idPrefix="template-screen-tab"
        onselect={(value) => (activeGroup = value as TemplateScreenGroupType)}
      />

      <div
        id="template-screen-panel"
        role="tabpanel"
        aria-labelledby={`template-screen-tab-${activeGroup}`}
        tabindex="0"
      >
        {#key activeGroup}
          <LazyComponent loader={screenLoaders[activeGroup]} />
        {/key}
      </div>
    </div>
  </div>
</div>
