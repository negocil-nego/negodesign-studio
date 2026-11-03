<script lang="ts">
  import SectionCarouselImageUsage from "./usage/SectionCarouselImageUsage.svelte";
  import svelteSource from "./usage/SectionCarouselImageUsage.svelte?raw";
  import reactSource from "./usage/SectionCarouselImageUsage.tsx?raw";
  import vueSource from "./usage/SectionCarouselImageUsage.vue?raw";
  import flutterSource from "./usage/SectionCarouselImageUsage.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";

</script>

<Terminal id="section-carousel">
  {#snippet preview()}
    <div class="mx-auto w-full max-w-sm">
      <SectionCarouselImageUsage />
    </div>
  {/snippet}

  {#snippet svelte()}<SourceCode path="src/components/local/ui/editor/terminal/carousel/usage/SectionCarouselImageUsage.svelte" source={svelteSource} />{/snippet}
  {#snippet react()}<SourceCode path="src/components/local/ui/editor/terminal/carousel/usage/SectionCarouselImageUsage.tsx" source={reactSource} />{/snippet}
  {#snippet vue()}<SourceCode path="src/components/local/ui/editor/terminal/carousel/usage/SectionCarouselImageUsage.vue" source={vueSource} />{/snippet}
  {#snippet flutter()}<SourceCode path="src/components/local/ui/editor/terminal/carousel/usage/SectionCarouselImageUsage.dart" source={flutterSource} />{/snippet}
</Terminal>
