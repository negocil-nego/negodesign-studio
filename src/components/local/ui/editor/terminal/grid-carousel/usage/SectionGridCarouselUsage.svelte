<script lang="ts">
  import SectionGridCarousel from "../../../../../../../project/ui/organisms/section/grid-carousel/ui/SectionGridCarousel.svelte";
  import CardMedia from "../../../../../../../project/ui/molecules/card/media/ui/CardMedia.svelte";
  import CardProduct from "../../../../../../../project/ui/molecules/card/product/ui/CardProduct.svelte";
</script>

<SectionGridCarousel
  titleProps={{ text: "SectionGridCarousel" }}
  descriptionProps={{ text: "Organize cartões numa apresentação responsiva." }}
  rows={2}
  columns={2}
>
  <CardMedia titleProps={{ title: "Design system" }} imageProps={{ src: "/img/ux-01.jpg", alt: "Equipa a trabalhar" }} />
  <CardProduct titleProps={{ title: "Componentes reutilizáveis" }} imageProps={{ src: "/img/programmer-01.jpg", alt: "Código num monitor" }} />
</SectionGridCarousel>
