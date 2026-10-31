<script lang="ts">
  import SectionCarouselImageUsage from "./SectionCarouselImageUsage.svelte";
  import svelteSource from "./SectionCarouselImageUsage.svelte?raw";
  import reactSource from "./SectionCarouselImageUsage.tsx?raw";
  import vueSource from "./SectionCarouselImageUsage.vue?raw";
  import flutterSource from "./SectionCarouselImageUsage.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";

</script>

<Terminal id="section-carousel">
  {#snippet preview()}
    <div class="mx-auto w-full max-w-sm">
      <SectionCarouselImageUsage />
    </div>
  {/snippet}

  {#snippet svelte()}<SourceCode path="src/components/local/ui/editor/terminal/carousel/SectionCarouselImageUsage.svelte" source={svelteSource} />{/snippet}
  {#snippet react()}<SourceCode path="src/components/local/ui/editor/terminal/carousel/SectionCarouselImageUsage.tsx" source={reactSource} />{/snippet}
  {#snippet vue()}<SourceCode path="src/components/local/ui/editor/terminal/carousel/SectionCarouselImageUsage.vue" source={vueSource} />{/snippet}
  {#snippet flutter()}<SourceCode path="src/components/local/ui/editor/terminal/carousel/SectionCarouselImageUsage.dart" source={flutterSource} />{/snippet}
</Terminal>
