<script lang="ts">
  import CarouselCardProfile from "../../../../../project/ui/organisms/section/carousel/card/profile/ui/CarouselCardProfile.svelte";
  import GridCardProfile from "../../../../../project/ui/organisms/section/grid/card/profile/ui/GridCardProfile.svelte";
  import GridCarouselCardProfile from "../../../../../project/ui/organisms/section/grid-carousel/card/profile/ui/GridCarouselCardProfile.svelte";
  import type { OpenApiEntity } from "../../../data/migration";

  let {
    entity,
    target,
  }: {
    entity: OpenApiEntity;
    target: "carousel" | "grid" | "grid-carousel";
  } = $props();

  const cards = $derived(entity.fields.map((field) => ({
    titleProps: { title: field.name },
    descriptionProps: { description: `${field.type}${field.required ? " · obrigatório" : ""}` },
    imageProps: { src: "/img/ux-01.jpg", alt: field.name },
  })));
</script>

{#if target === "carousel"}
  <CarouselCardProfile
    sectionCarouselButtonsProps={{ titleProps: { text: entity.name }, descriptionProps: { text: "Response OpenAPI apresentada em carousel." } }}
    items={cards}
  />
{:else if target === "grid"}
  <GridCardProfile
    sectionGridProps={{ titleProps: { text: entity.name }, descriptionProps: { text: "Response OpenAPI apresentada em grelha." } }}
    items={cards}
  />
{:else if target === "grid-carousel"}
  <GridCarouselCardProfile
    sectionGridCarouselProps={{ titleProps: { text: entity.name }, descriptionProps: { text: "Response OpenAPI apresentada em grelha navegável." }, rows: 1, columns: 3 }}
    items={cards}
  />
{/if}
