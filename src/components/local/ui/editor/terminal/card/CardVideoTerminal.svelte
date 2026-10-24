<script lang="ts">
  import CardVideo from "../../../../../../project/ui/molecules/card/shared/card-video/ui/CardVideo.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/card/shared/card-video/ui/CardVideo.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/card/shared/card-video/ui/CardVideo.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/card/shared/card-video/ui/CardVideo.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/card/shared/card-video/ui/CardVideo.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="card-video">
  {#snippet preview()}
    <div class="w-56 overflow-hidden rounded-lg">
      <CardVideo src="" poster="/img/ux-01.jpg" preload="none" controls />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-video/ui/CardVideo.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-video/ui/CardVideo.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-video/ui/CardVideo.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/molecules/card/shared/card-video/ui/CardVideo.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
