<script lang="ts">
  import GridCardProfile from "../../../../../../project/ui/organisms/section/grid/card/profile/ui/GridCardProfile.svelte";
  import svelteSource from "./usage/GridCardProfileUsage.svelte?raw";
  import reactSource from "./usage/GridCardProfileUsage.tsx?raw";
  import vueSource from "./usage/GridCardProfileUsage.vue?raw";
  import flutterSource from "./usage/GridCardProfileUsage.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";

  const items = [
    {
      titleProps: { title: "Design system" },
      imageProps: { src: "/img/ux-01.jpg", alt: "Design system" },
    },
    {
      titleProps: { title: "Componentes reutilizáveis" },
      imageProps: { src: "/img/programmer-01.jpg", alt: "Componentes reutilizáveis" },
    },
    {
      titleProps: { title: "Experiências digitais" },
      imageProps: { src: "/img/programmer-02.jpg", alt: "Experiências digitais" },
    },
    {
      titleProps: { title: "Interfaces adaptáveis" },
      imageProps: { src: "/img/ux-01.jpg", alt: "Interfaces adaptáveis" },
    },
  ];
</script>

<Terminal id="grid-card-profile">
  {#snippet preview()}
    <div class="rounded-lg border border-blender-border bg-blender-base p-5 sm:p-7">
      <GridCardProfile
        sectionGridProps={{
          titleProps: { text: "GridCardProfile" },
          descriptionProps: { text: "Cards apresentados numa grelha responsiva." },
        }}
        {items}
      />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/components/local/ui/editor/terminal/grid/usage/GridCardProfileUsage.svelte"
      source={svelteSource}
    />
  {/snippet}
  {#snippet react()}
    <SourceCode
      path="src/components/local/ui/editor/terminal/grid/usage/GridCardProfileUsage.tsx"
      source={reactSource}
    />
  {/snippet}
  {#snippet vue()}
    <SourceCode
      path="src/components/local/ui/editor/terminal/grid/usage/GridCardProfileUsage.vue"
      source={vueSource}
    />
  {/snippet}
  {#snippet flutter()}
    <SourceCode
      path="src/components/local/ui/editor/terminal/grid/usage/GridCardProfileUsage.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
