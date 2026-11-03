<script lang="ts">
  import GridCarouselCardProfile from "../../../../../../project/ui/organisms/section/grid-carousel/card/profile/ui/GridCarouselCardProfile.svelte";
  import svelteSource from "../../../../../../project/ui/organisms/section/grid-carousel/card/profile/ui/GridCarouselCardProfile.svelte?raw";
  import reactSource from "../../../../../../project/ui/organisms/section/grid-carousel/card/profile/ui/GridCarouselCardProfile.tsx?raw";
  import vueSource from "../../../../../../project/ui/organisms/section/grid-carousel/card/profile/ui/GridCarouselCardProfile.vue?raw";
  import flutterSource from "../../../../../../project/ui/organisms/section/grid-carousel/card/profile/ui/GridCarouselCardProfile.dart?raw";
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

<Terminal id="grid-carousel-card-profile">
  {#snippet preview()}
    <div class="rounded-lg border border-blender-border bg-blender-base p-5 sm:p-7">
      <GridCarouselCardProfile
        sectionGridCarouselProps={{
          titleProps: { text: "GridCarouselCardProfile" },
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
      path="src/project/ui/organisms/section/grid-carousel/card/profile/ui/GridCarouselCardProfile.svelte"
      source={svelteSource}
    />
  {/snippet}
  {#snippet react()}
    <SourceCode
      path="src/project/ui/organisms/section/grid-carousel/card/profile/ui/GridCarouselCardProfile.tsx"
      source={reactSource}
    />
  {/snippet}
  {#snippet vue()}
    <SourceCode
      path="src/project/ui/organisms/section/grid-carousel/card/profile/ui/GridCarouselCardProfile.vue"
      source={vueSource}
    />
  {/snippet}
  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/organisms/section/grid-carousel/card/profile/ui/GridCarouselCardProfile.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
