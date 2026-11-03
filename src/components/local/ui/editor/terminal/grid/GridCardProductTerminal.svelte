<script lang="ts">
  import GridCardProduct from "../../../../../../project/ui/organisms/section/grid/card/product/ui/GridCardProduct.svelte";
  import svelteSource from "../../../../../../project/ui/organisms/section/grid/card/product/ui/GridCardProduct.svelte?raw";
  import reactSource from "../../../../../../project/ui/organisms/section/grid/card/product/ui/GridCardProduct.tsx?raw";
  import vueSource from "../../../../../../project/ui/organisms/section/grid/card/product/ui/GridCardProduct.vue?raw";
  import flutterSource from "../../../../../../project/ui/organisms/section/grid/card/product/ui/GridCardProduct.dart?raw";
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

<Terminal id="grid-card-product">
  {#snippet preview()}
    <div class="rounded-lg border border-blender-border bg-blender-base p-5 sm:p-7">
      <GridCardProduct
        sectionGridProps={{
          titleProps: { text: "GridCardProduct" },
          descriptionProps: { text: "Cards apresentados numa grelha responsiva." },
        }}
        {items}
      />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/organisms/section/grid/card/product/ui/GridCardProduct.svelte"
      source={svelteSource}
    />
  {/snippet}
  {#snippet react()}
    <SourceCode
      path="src/project/ui/organisms/section/grid/card/product/ui/GridCardProduct.tsx"
      source={reactSource}
    />
  {/snippet}
  {#snippet vue()}
    <SourceCode
      path="src/project/ui/organisms/section/grid/card/product/ui/GridCardProduct.vue"
      source={vueSource}
    />
  {/snippet}
  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/organisms/section/grid/card/product/ui/GridCardProduct.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
