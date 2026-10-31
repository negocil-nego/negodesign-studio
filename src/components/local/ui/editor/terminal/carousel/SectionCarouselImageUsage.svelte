<script lang="ts">
  import SectionCarousel from "../../../../../../project/ui/organisms/section/carousel/image/ui/SectionCarousel.svelte";

  const items = [
    {
      backgroundImage: "/img/programmer-01.jpg",
      title: { text: "Explore novos lugares" },
      description: { text: "Descubra experiências e ideias para a sua próxima viagem." },
    },
    {
      backgroundImage: "/img/ux-01.jpg",
      title: { text: "Inspire-se todos os dias" },
      description: { text: "Encontre histórias que ajudam a transformar os seus projectos." },
    },
    {
      backgroundImage: "/img/programmer-02.jpg",
      title: { text: "Partilhe o seu próximo destino" },
      description: { text: "Guarde os momentos e partilhe-os com a sua comunidade." },
    },
  ];
</script>

<div class="mx-auto w-full max-w-sm">
  <SectionCarousel {items} backgroundColor="bg-gradient-to-t from-blue-950/70 to-transparent" />
</div>
