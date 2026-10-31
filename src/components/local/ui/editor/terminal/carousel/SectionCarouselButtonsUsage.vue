<script setup lang="ts">
import SectionCarouselButtons from "../../../../../../project/ui/organisms/section/carousel/buttons/ui/SectionCarouselButtons.vue";

const testimonials = [
  { name: "Blake Star", role: "COO, blackstar.com", image: "/img/programmer-01.jpg", rating: 5 },
  { name: "David Gilmore", role: "CTO, darkside.ai", image: "/img/ux-01.jpg", rating: 5 },
  { name: "Gerard White", role: "COO, exo4emo.ai", image: "/img/programmer-02.jpg", rating: 5 },
  { name: "Azuny", role: "COO, studio.example", image: "/img/ux-01.jpg", rating: 4 },
];
</script>

<template>
  <SectionCarouselButtons
    :title-props="{ text: 'O que dizem os nossos clientes' }"
    :description-props="{ text: 'Histórias de pessoas que usam a plataforma no dia-a-dia.' }"
  >
    <article v-for="person in testimonials" :key="person.name" class="overflow-hidden rounded-3xl border border-blender-border bg-blender-card">
      <img :src="person.image" :alt="person.name" class="h-56 w-full object-cover" />
      <div class="space-y-2 p-4">
        <p class="text-amber-400" :aria-label="`${person.rating} estrelas`">{{ '★'.repeat(person.rating) }}{{ '☆'.repeat(5 - person.rating) }}</p>
        <h3 class="font-semibold text-zinc-100">{{ person.name }}</h3>
        <p class="text-xs text-zinc-400">{{ person.role }}</p>
      </div>
    </article>
  </SectionCarouselButtons>
</template>
