<script lang="ts">
  import GridCardMedia from "../../../../../../project/ui/organisms/section/grid/card/media/ui/GridCardMedia.svelte";
  import svelteSource from "./usage/GridCardMediaUsage.svelte?raw";
  import reactSource from "./usage/GridCardMediaUsage.tsx?raw";
  import vueSource from "./usage/GridCardMediaUsage.vue?raw";
  import flutterSource from "./usage/GridCardMediaUsage.dart?raw";
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

<Terminal id="grid-card-media">
  {#snippet preview()}
    <div class="rounded-lg border border-blender-border bg-blender-base p-5 sm:p-7">
      <GridCardMedia
        sectionGridProps={{
          titleProps: { text: "GridCardMedia" },
          descriptionProps: { text: "Cards apresentados numa grelha responsiva." },
        }}
        {items}
      />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/components/local/ui/editor/terminal/grid/usage/GridCardMediaUsage.svelte"
      source={svelteSource}
    />
  {/snippet}
  {#snippet react()}
    <SourceCode
      path="src/components/local/ui/editor/terminal/grid/usage/GridCardMediaUsage.tsx"
      source={reactSource}
    />
  {/snippet}
  {#snippet vue()}
    <SourceCode
      path="src/components/local/ui/editor/terminal/grid/usage/GridCardMediaUsage.vue"
      source={vueSource}
    />
  {/snippet}
  {#snippet flutter()}
    <SourceCode
      path="src/components/local/ui/editor/terminal/grid/usage/GridCardMediaUsage.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
