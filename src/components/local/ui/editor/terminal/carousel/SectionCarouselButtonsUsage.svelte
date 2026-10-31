<script lang="ts">
  import SectionCarouselButtons from "../../../../../../project/ui/organisms/section/carousel/buttons/ui/SectionCarouselButtons.svelte";
</script>

<SectionCarouselButtons
  titleProps={{ text: "O que dizem os nossos clientes" }}
  descriptionProps={{ text: "Histórias de pessoas que usam a plataforma no dia-a-dia." }}
>
  <article class="overflow-hidden rounded-3xl border border-blender-border bg-blender-card">
    <img src="/img/programmer-01.jpg" alt="Blake Star" class="h-56 w-full object-cover" />
    <div class="space-y-2 p-4"><p class="text-amber-400" aria-label="5 estrelas">★★★★★</p><h3 class="font-semibold text-zinc-100">Blake Star</h3><p class="text-xs text-zinc-400">COO, blackstar.com</p></div>
  </article>
  <article class="overflow-hidden rounded-3xl border border-blender-border bg-blender-card">
    <img src="/img/ux-01.jpg" alt="David Gilmore" class="h-56 w-full object-cover" />
    <div class="space-y-2 p-4"><p class="text-amber-400" aria-label="5 estrelas">★★★★★</p><h3 class="font-semibold text-zinc-100">David Gilmore</h3><p class="text-xs text-zinc-400">CTO, darkside.ai</p></div>
  </article>
  <article class="overflow-hidden rounded-3xl border border-blender-border bg-blender-card">
    <img src="/img/programmer-02.jpg" alt="Gerard White" class="h-56 w-full object-cover" />
    <div class="space-y-2 p-4"><p class="text-amber-400" aria-label="5 estrelas">★★★★★</p><h3 class="font-semibold text-zinc-100">Gerard White</h3><p class="text-xs text-zinc-400">COO, exo4emo.ai</p></div>
  </article>
  <article class="overflow-hidden rounded-3xl border border-blender-border bg-blender-card">
    <img src="/img/ux-01.jpg" alt="Azuny" class="h-56 w-full object-cover" />
    <div class="space-y-2 p-4"><p class="text-amber-400" aria-label="4 estrelas">★★★★☆</p><h3 class="font-semibold text-zinc-100">Azuny</h3><p class="text-xs text-zinc-400">COO, studio.example</p></div>
  </article>
</SectionCarouselButtons>
