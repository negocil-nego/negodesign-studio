<script lang="ts">
  import GridCarouselCardMedia from "../../../../../../project/ui/organisms/section/grid-carousel/card/media/ui/GridCarouselCardMedia.svelte";
  import svelteSource from "../../../../../../project/ui/organisms/section/grid-carousel/card/media/ui/GridCarouselCardMedia.svelte?raw";
  import reactSource from "../../../../../../project/ui/organisms/section/grid-carousel/card/media/ui/GridCarouselCardMedia.tsx?raw";
  import vueSource from "../../../../../../project/ui/organisms/section/grid-carousel/card/media/ui/GridCarouselCardMedia.vue?raw";
  import flutterSource from "../../../../../../project/ui/organisms/section/grid-carousel/card/media/ui/GridCarouselCardMedia.dart?raw";
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

<Terminal id="grid-carousel-card-media">
  {#snippet preview()}
    <div class="rounded-lg border border-blender-border bg-blender-base p-5 sm:p-7">
      <GridCarouselCardMedia
        sectionGridCarouselProps={{
          titleProps: { text: "GridCarouselCardMedia" },
          descriptionProps: { text: "Cards em páginas com linhas e colunas configuráveis." },
          rows: 2,
          columns: 2,
        }}
        {items}
      />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/organisms/section/grid-carousel/card/media/ui/GridCarouselCardMedia.svelte"
      source={svelteSource}
    />
  {/snippet}
  {#snippet react()}
    <SourceCode
      path="src/project/ui/organisms/section/grid-carousel/card/media/ui/GridCarouselCardMedia.tsx"
      source={reactSource}
    />
  {/snippet}
  {#snippet vue()}
    <SourceCode
      path="src/project/ui/organisms/section/grid-carousel/card/media/ui/GridCarouselCardMedia.vue"
      source={vueSource}
    />
  {/snippet}
  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/organisms/section/grid-carousel/card/media/ui/GridCarouselCardMedia.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
