<script lang="ts">
  import CardMedia from "../../../../../../project/ui/molecules/card/media/ui/CardMedia.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/card/media/ui/CardMedia.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/card/media/ui/CardMedia.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/card/media/ui/CardMedia.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/card/media/ui/CardMedia.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";

  let variant = $state<"01" | "02">("01");
</script>

<Terminal id="card-media">
  {#snippet preview()}
    <div class="w-full space-y-3">
      <label class="flex items-center gap-2 text-xs text-zinc-400">
        Variante
        <select bind:value={variant} class="rounded border border-blender-border bg-blender-card px-2 py-1 text-zinc-100">
          <option value="01">01</option>
          <option value="02">02</option>
        </select>
      </label>
      <div class="w-full max-w-xs">
      <CardMedia
        {variant}
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
