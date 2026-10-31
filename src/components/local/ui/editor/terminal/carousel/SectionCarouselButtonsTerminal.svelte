<script lang="ts">
  import SectionCarouselButtonsUsage from "./SectionCarouselButtonsUsage.svelte";
  import svelteSource from "./SectionCarouselButtonsUsage.svelte?raw";
  import reactSource from "./SectionCarouselButtonsUsage.tsx?raw";
  import vueSource from "./SectionCarouselButtonsUsage.vue?raw";
  import flutterSource from "./SectionCarouselButtonsUsage.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="section-carousel-buttons">
  {#snippet preview()}
    <div
      class="rounded-lg border border-blender-border bg-blender-base p-5 sm:p-7"
    >
      <SectionCarouselButtonsUsage />
    </div>
  {/snippet}

  {#snippet svelte()}<SourceCode
      path="src/components/local/ui/editor/terminal/carousel/SectionCarouselButtonsUsage.svelte"
      source={svelteSource}
    />{/snippet}
  {#snippet react()}<SourceCode
      path="src/components/local/ui/editor/terminal/carousel/SectionCarouselButtonsUsage.tsx"
      source={reactSource}
    />{/snippet}
  {#snippet vue()}<SourceCode
      path="src/components/local/ui/editor/terminal/carousel/SectionCarouselButtonsUsage.vue"
      source={vueSource}
    />{/snippet}
  {#snippet flutter()}<SourceCode
      path="src/components/local/ui/editor/terminal/carousel/SectionCarouselButtonsUsage.dart"
      source={flutterSource}
    />{/snippet}
</Terminal>
