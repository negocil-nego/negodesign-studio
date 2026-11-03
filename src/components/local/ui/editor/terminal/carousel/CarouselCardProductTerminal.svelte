<script lang="ts">
  import CarouselCardProduct from "../../../../../../project/ui/organisms/section/carousel/card/product/ui/CarouselCardProduct.svelte";
  import svelteSource from "../../../../../../project/ui/organisms/section/carousel/card/product/ui/CarouselCardProduct.svelte?raw";
  import reactSource from "../../../../../../project/ui/organisms/section/carousel/card/product/ui/CarouselCardProduct.tsx?raw";
  import vueSource from "../../../../../../project/ui/organisms/section/carousel/card/product/ui/CarouselCardProduct.vue?raw";
  import flutterSource from "../../../../../../project/ui/organisms/section/carousel/card/product/ui/CarouselCardProduct.dart?raw";
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

<Terminal id="carousel-card-product">
  {#snippet preview()}
    <div class="rounded-lg border border-blender-border bg-blender-base p-5 sm:p-7">
      <CarouselCardProduct
        sectionCarouselButtonsProps={{
          titleProps: { text: "CarouselCardProduct" },
          descriptionProps: { text: "Quatro cards apresentados no carousel." },
        }}
        {items}
      />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/organisms/section/carousel/card/product/ui/CarouselCardProduct.svelte"
      source={svelteSource}
    />
  {/snippet}
  {#snippet react()}
    <SourceCode
      path="src/project/ui/organisms/section/carousel/card/product/ui/CarouselCardProduct.tsx"
      source={reactSource}
    />
  {/snippet}
  {#snippet vue()}
    <SourceCode
      path="src/project/ui/organisms/section/carousel/card/product/ui/CarouselCardProduct.vue"
      source={vueSource}
    />
  {/snippet}
  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/organisms/section/carousel/card/product/ui/CarouselCardProduct.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
