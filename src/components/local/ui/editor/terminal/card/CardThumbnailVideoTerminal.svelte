<script lang="ts">
  import CardThumbnailVideo from "../../../../../../project/ui/molecules/card/shared/card-thumbnail-video/ui/CardThumbnailVideo.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/card/shared/card-thumbnail-video/ui/CardThumbnailVideo.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/card/shared/card-thumbnail-video/ui/CardThumbnailVideo.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/card/shared/card-thumbnail-video/ui/CardThumbnailVideo.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/card/shared/card-thumbnail-video/ui/CardThumbnailVideo.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="card-thumbnail-video">
  {#snippet preview()}
    <div class="w-56">
      <CardThumbnailVideo
        videoProps={{ src: "" }}
        imageProps={{ src: "/img/ux-01.jpg", alt: "Miniatura de vídeo" }}
        visibleButtonPlay
      />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-thumbnail-video/ui/CardThumbnailVideo.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-thumbnail-video/ui/CardThumbnailVideo.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-thumbnail-video/ui/CardThumbnailVideo.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-thumbnail-video/ui/CardThumbnailVideo.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
