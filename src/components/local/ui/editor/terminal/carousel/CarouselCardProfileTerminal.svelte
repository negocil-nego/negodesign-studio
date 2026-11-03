<script lang="ts">
  import CarouselCardProfile from "../../../../../../project/ui/organisms/section/carousel/card/profile/ui/CarouselCardProfile.svelte";
  import svelteSource from "../../../../../../project/ui/organisms/section/carousel/card/profile/ui/CarouselCardProfile.svelte?raw";
  import reactSource from "../../../../../../project/ui/organisms/section/carousel/card/profile/ui/CarouselCardProfile.tsx?raw";
  import vueSource from "../../../../../../project/ui/organisms/section/carousel/card/profile/ui/CarouselCardProfile.vue?raw";
  import flutterSource from "../../../../../../project/ui/organisms/section/carousel/card/profile/ui/CarouselCardProfile.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";

  const items = [
    {
      titleProps: { title: "Design system" },
      imageProps: { src: "/img/ux-01.jpg", alt: "Design system" },
    },
    {
      titleProps: { title: "Componentes reutilizáveis" },
      imageProps: { src: "/img/programmer-01.jpg", alt: "Componentes reutilizáveis" },
    },
    {
      titleProps: { title: "Experiências digitais" },
      imageProps: { src: "/img/programmer-02.jpg", alt: "Experiências digitais" },
    },
    {
      titleProps: { title: "Interfaces adaptáveis" },
      imageProps: { src: "/img/ux-01.jpg", alt: "Interfaces adaptáveis" },
    },
  ];
</script>

<Terminal id="carousel-card-profile">
  {#snippet preview()}
    <div class="rounded-lg border border-blender-border bg-blender-base p-5 sm:p-7">
      <CarouselCardProfile
        sectionCarouselButtonsProps={{
          titleProps: { text: "CarouselCardProfile" },
          descriptionProps: { text: "Quatro cards apresentados no carousel." },
        }}
        {items}
      />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/organisms/section/carousel/card/profile/ui/CarouselCardProfile.svelte"
      source={svelteSource}
    />
  {/snippet}
  {#snippet react()}
    <SourceCode
      path="src/project/ui/organisms/section/carousel/card/profile/ui/CarouselCardProfile.tsx"
      source={reactSource}
    />
  {/snippet}
  {#snippet vue()}
    <SourceCode
      path="src/project/ui/organisms/section/carousel/card/profile/ui/CarouselCardProfile.vue"
      source={vueSource}
    />
  {/snippet}
  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/organisms/section/carousel/card/profile/ui/CarouselCardProfile.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
