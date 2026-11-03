<script lang="ts">
  import SectionGrid from "../../../../../../project/ui/organisms/section/grid/ui/SectionGrid.svelte";
  import svelteSource from "../../../../../../project/ui/organisms/section/grid/ui/SectionGrid.svelte?raw";
  import reactSource from "../../../../../../project/ui/organisms/section/grid/ui/SectionGrid.tsx?raw";
  import vueSource from "../../../../../../project/ui/organisms/section/grid/ui/SectionGrid.vue?raw";
  import flutterSource from "../../../../../../project/ui/organisms/section/grid/ui/SectionGrid.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
  import CardMedia from "../../../../../../project/ui/molecules/card/media/ui/CardMedia.svelte";
  import CardProduct from "../../../../../../project/ui/molecules/card/product/ui/CardProduct.svelte";
  import CardProfile from "../../../../../../project/ui/molecules/card/profile/ui/CardProfile.svelte";
</script>

<Terminal id="section-grid">
  {#snippet preview()}
    <div class="rounded-lg border border-blender-border bg-blender-base p-5 sm:p-7">
      <SectionGrid
        titleProps={{ text: "SectionGrid" }}
        descriptionProps={{ text: "Grelha responsiva com cartões de exemplo." }}
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
      </SectionGrid>
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/organisms/section/grid/ui/SectionGrid.svelte"
      source={svelteSource}
    />
  {/snippet}
  {#snippet react()}
    <SourceCode
      path="src/project/ui/organisms/section/grid/ui/SectionGrid.tsx"
      source={reactSource}
    />
  {/snippet}
  {#snippet vue()}
    <SourceCode
      path="src/project/ui/organisms/section/grid/ui/SectionGrid.vue"
      source={vueSource}
    />
  {/snippet}
  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/organisms/section/grid/ui/SectionGrid.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
