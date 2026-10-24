<script lang="ts">
  import CardMedia from "../../../../../../project/ui/molecules/card/media/ui/CardMedia.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/card/media/ui/CardMedia.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/card/media/ui/CardMedia.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/card/media/ui/CardMedia.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/card/media/ui/CardMedia.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="card-media">
  {#snippet preview()}
    <div class="w-full max-w-xs">
      <CardMedia
        variant="01"
        logoProps={{ src: "/img/programmer-01.jpg", alt: "Marca" }}
        titleProps={{ title: "Auscultadores Pro" }}
        ratingProps={{ rating: 4 }}
        priceProps={{ number: 129.9 }}
        descriptionProps={{ description: "Som imersivo com cancelamento de ruído." }}
        imageProps={{ src: "/img/ux-01.jpg", alt: "Produto" }}
        buttonProps={{ text: "Ver detalhes" }}
        cartButtonProps={{}}
        favoriteButtonProps={{}}
      />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/molecules/card/media/ui/CardMedia.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/molecules/card/media/ui/CardMedia.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/molecules/card/media/ui/CardMedia.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/molecules/card/media/ui/CardMedia.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
