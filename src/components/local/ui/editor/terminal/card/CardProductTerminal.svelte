<script lang="ts">
  import CardProduct from "../../../../../../project/ui/molecules/card/product/ui/CardProduct.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/card/product/ui/CardProduct.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/card/product/ui/CardProduct.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/card/product/ui/CardProduct.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/card/product/ui/CardProduct.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";

  import type { CardMinFileProps } from "../../../../../../project/ui/molecules/card/shared/card-min-file/data/types";

  const cardMinFiles: CardMinFileProps[] = [
    { imageProps: { src: "/img/ux-01.jpg", alt: "Miniatura 1" }, active: true },
    { imageProps: { src: "/img/programmer-01.jpg", alt: "Miniatura 2" } },
    { imageProps: { src: "/img/programmer-02.jpg", alt: "Miniatura 3" } },
  ];
</script>

<Terminal id="card-product">
  {#snippet preview()}
    <div class="w-full max-w-xs">
      <CardProduct
        variant="01"
        logoProps={{ src: "/img/programmer-01.jpg", alt: "Marca" }}
        imageProps={{ src: "/img/ux-01.jpg", alt: "Produto" }}
        titleProps={{ title: "Teclado Mecânico" }}
        ratingProps={{ rating: 5 }}
        priceProps={{ number: 349.9 }}
        descriptionProps={{ description: "Switch táctil com retroiluminação RGB." }}
        minFileProps={cardMinFiles}
        buttonProps={{ text: "Comprar" }}
        cartButtonProps={{}}
        favoriteButtonProps={{}}
      />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/molecules/card/product/ui/CardProduct.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/molecules/card/product/ui/CardProduct.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/molecules/card/product/ui/CardProduct.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/molecules/card/product/ui/CardProduct.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
