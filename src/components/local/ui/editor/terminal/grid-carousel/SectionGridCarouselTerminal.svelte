<script lang="ts">
  import SectionGridCarousel from "../../../../../../project/ui/organisms/section/grid-carousel/ui/SectionGridCarousel.svelte";
  import svelteSource from "../../../../../../project/ui/organisms/section/grid-carousel/ui/SectionGridCarousel.svelte?raw";
  import reactSource from "../../../../../../project/ui/organisms/section/grid-carousel/ui/SectionGridCarousel.tsx?raw";
  import vueSource from "../../../../../../project/ui/organisms/section/grid-carousel/ui/SectionGridCarousel.vue?raw";
  import flutterSource from "../../../../../../project/ui/organisms/section/grid-carousel/ui/SectionGridCarousel.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
  import CardMedia from "../../../../../../project/ui/molecules/card/media/ui/CardMedia.svelte";
  import CardProduct from "../../../../../../project/ui/molecules/card/product/ui/CardProduct.svelte";
  import CardProfile from "../../../../../../project/ui/molecules/card/profile/ui/CardProfile.svelte";
</script>

<Terminal id="section-grid-carousel">
  {#snippet preview()}
    <div class="rounded-lg border border-blender-border bg-blender-base p-5 sm:p-7">
      <SectionGridCarousel
        titleProps={{ text: "SectionGridCarousel" }}
        descriptionProps={{ text: "Grelha deslizável com duas linhas e duas colunas." }}
        rows={2}
        columns={2}
      >
        <CardMedia
          titleProps={{ title: "CardMedia de exemplo" }}
          imageProps={{ src: "/img/ux-01.jpg", alt: "CardMedia" }}
        />
        <CardProduct
          titleProps={{ title: "CardProduct de exemplo" }}
          imageProps={{ src: "/img/ux-01.jpg", alt: "CardProduct" }}
        />
        <CardProfile
          titleProps={{ title: "CardProfile de exemplo" }}
          imageProps={{ src: "/img/ux-01.jpg", alt: "CardProfile" }}
        />
      </SectionGridCarousel>
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/organisms/section/grid-carousel/ui/SectionGridCarousel.svelte"
      source={svelteSource}
    />
  {/snippet}
  {#snippet react()}
    <SourceCode
      path="src/project/ui/organisms/section/grid-carousel/ui/SectionGridCarousel.tsx"
      source={reactSource}
    />
  {/snippet}
  {#snippet vue()}
    <SourceCode
      path="src/project/ui/organisms/section/grid-carousel/ui/SectionGridCarousel.vue"
      source={vueSource}
    />
  {/snippet}
  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/organisms/section/grid-carousel/ui/SectionGridCarousel.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
