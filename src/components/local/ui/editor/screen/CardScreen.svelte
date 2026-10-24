<script lang="ts">
  import CardMediaTerminal from "../terminal/card/CardMediaTerminal.svelte";
  import CardProfileTerminal from "../terminal/card/CardProfileTerminal.svelte";
  import CardProductTerminal from "../terminal/card/CardProductTerminal.svelte";
  import CardTerminal from "../terminal/card/CardTerminal.svelte";
  import CardImageTerminal from "../terminal/card/CardImageTerminal.svelte";
  import CardTitleTerminal from "../terminal/card/CardTitleTerminal.svelte";
  import CardDescriptionTerminal from "../terminal/card/CardDescriptionTerminal.svelte";
  import CardPriceTerminal from "../terminal/card/CardPriceTerminal.svelte";
  import CardRatingTerminal from "../terminal/card/CardRatingTerminal.svelte";
  import CardTagsTerminal from "../terminal/card/CardTagsTerminal.svelte";
  import CardLogoTerminal from "../terminal/card/CardLogoTerminal.svelte";
  import CardPhoneTerminal from "../terminal/card/CardPhoneTerminal.svelte";
  import CardVideoTerminal from "../terminal/card/CardVideoTerminal.svelte";
  import CardThumbnailVideoTerminal from "../terminal/card/CardThumbnailVideoTerminal.svelte";
  import CardMinFileTerminal from "../terminal/card/CardMinFileTerminal.svelte";
  import CardButtonTerminal from "../terminal/card/CardButtonTerminal.svelte";
  import CardCartButtonTerminal from "../terminal/card/CardCartButtonTerminal.svelte";
  import CardFavoriteButtonTerminal from "../terminal/card/CardFavoriteButtonTerminal.svelte";
  import CardActionsButtonTerminal from "../terminal/card/CardActionsButtonTerminal.svelte";
</script>

<div class="space-y-5">
  <div>
    <p class="text-[10px] font-semibold uppercase tracking-[0.18em] text-blue-400">
      Moléculas / Card
    </p>
    <h2 class="mt-1 text-lg font-semibold text-zinc-100">Cartões</h2>
    <p class="mt-2 text-xs leading-5 text-zinc-400">
      Pré-visualização interactiva e código-fonte dos componentes de cartão.
    </p>
  </div>

  <CardMediaTerminal />
  <CardProfileTerminal />
  <CardProductTerminal />
  <CardTerminal />
  <CardImageTerminal />
  <CardTitleTerminal />
  <CardDescriptionTerminal />
  <CardPriceTerminal />
  <CardRatingTerminal />
  <CardTagsTerminal />
  <CardLogoTerminal />
  <CardPhoneTerminal />
  <CardVideoTerminal />
  <CardThumbnailVideoTerminal />
  <CardMinFileTerminal />
  <CardButtonTerminal />
  <CardCartButtonTerminal />
  <CardFavoriteButtonTerminal />
  <CardActionsButtonTerminal />
</div>
