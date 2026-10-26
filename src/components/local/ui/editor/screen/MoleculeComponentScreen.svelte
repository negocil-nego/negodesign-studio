<script lang="ts">
  import CountryPhonePopover from "../../../../../project/ui/molecules/popover/ui/CountryPhonePopover.svelte";
  import CardMedia from "../../../../../project/ui/molecules/card/media/ui/CardMedia.svelte";
  import CardProfile from "../../../../../project/ui/molecules/card/profile/ui/CardProfile.svelte";
  import CardProduct from "../../../../../project/ui/molecules/card/product/ui/CardProduct.svelte";
  import Card from "../../../../../project/ui/molecules/card/shared/card/ui/Card.svelte";
  import CardImage from "../../../../../project/ui/molecules/card/shared/card-image/ui/CardImage.svelte";
  import CardTitle from "../../../../../project/ui/molecules/card/shared/card-title/ui/CardTitle.svelte";
  import CardDescription from "../../../../../project/ui/molecules/card/shared/card-description/ui/CardDescription.svelte";
  import CardPrice from "../../../../../project/ui/molecules/card/shared/card-price/ui/CardPrice.svelte";
  import CardRating from "../../../../../project/ui/molecules/card/shared/card-rating/ui/CardRating.svelte";
  import CardTags from "../../../../../project/ui/molecules/card/shared/card-tags/ui/CardTags.svelte";
  import CardLogo from "../../../../../project/ui/molecules/card/shared/card-logo/ui/CardLogo.svelte";
  import CardPhone from "../../../../../project/ui/molecules/card/shared/card-phone/ui/CardPhone.svelte";
  import CardVideo from "../../../../../project/ui/molecules/card/shared/card-video/ui/CardVideo.svelte";
  import CardThumbnailVideo from "../../../../../project/ui/molecules/card/shared/card-thumbnail-video/ui/CardThumbnailVideo.svelte";
  import CardMinFile from "../../../../../project/ui/molecules/card/shared/card-min-file/ui/CardMinFile.svelte";
  import CardButton from "../../../../../project/ui/molecules/card/shared/card-button/ui/CardButton.svelte";
  import CardCartButton from "../../../../../project/ui/molecules/card/shared/card-cart-button/ui/CardCartButton.svelte";
  import CardFavoriteButton from "../../../../../project/ui/molecules/card/shared/card-favorite-button/ui/CardFavoriteButton.svelte";
  import CardActionsButton from "../../../../../project/ui/molecules/card/shared/card-actions-button/ui/CardActionsButton.svelte";
  import { FlagNamed } from "../../../../../hooks/flag.svelte";
  import type { ComponentDefinition } from "../../../data/components";
  import { activeProject } from "../../../data/projects";
  import type { CardTag } from "../../../../../project/ui/molecules/card/shared/card-tags/data/types";
  import type { CardActionButton } from "../../../../../project/ui/molecules/card/shared/card-actions-button/data/types";
  import type { CardMinFileProps } from "../../../../../project/ui/molecules/card/shared/card-min-file/data/types";

  let {
    components,
    copyingId,
    onCopy,
  }: {
    components: ComponentDefinition[];
    copyingId: string | null;
    onCopy: (component: ComponentDefinition) => void;
  } = $props();

  const sharedCardComponents = $derived(
    components.filter((component) =>
      component.sourceDirectory?.includes("/card/shared/"),
    ),
  );
  const visibleComponents = $derived(
    components.filter(
      (component) => !component.sourceDirectory?.includes("/card/shared/"),
    ),
  );

  const cardTags: CardTag[] = [
    { icon: "Star", title: "Novo" },
    { icon: "Location", title: "Luanda" },
  ];

  const cardActions: CardActionButton[] = [
    { icon: "Link", label: "Partilhar" },
    { icon: "MoreHorizontal", label: "Mais" },
  ];

  const cardMinFiles: CardMinFileProps[] = [
    { imageProps: { src: "/img/ux-01.jpg", alt: "Miniatura 1" }, active: true },
    { imageProps: { src: "/img/programmer-01.jpg", alt: "Miniatura 2" } },
    { imageProps: { src: "/img/programmer-02.jpg", alt: "Miniatura 3" } },
  ];
</script>

{#snippet componentCard(component: ComponentDefinition)}
  <article
    class="overflow-hidden rounded-xl border border-blender-border bg-blender-card shadow-sm transition-shadow hover:shadow-md"
  >
    <div class="flex min-h-36 items-center justify-center bg-blender-base p-5">
      {#if component.id === "card-media"}
        <div class="w-full max-w-xs">
          <CardMedia
            variant="01"
            logoProps={{ src: "/img/programmer-01.jpg", alt: "Marca" }}
            titleProps={{ title: "Auscultadores Pro" }}
            ratingProps={{ rating: 4 }}
            priceProps={{ number: 129.9 }}
            descriptionProps={{
              description: "Som imersivo com cancelamento de ruído.",
            }}
            imageProps={{ src: "/img/ux-01.jpg", alt: "Produto" }}
            buttonProps={{ text: "Ver detalhes" }}
            cartButtonProps={{}}
            favoriteButtonProps={{}}
          />
        </div>
      {:else if component.id === "card-profile"}
        <div class="w-full max-w-xs">
          <CardProfile
            variant="01"
            logoProps={{ src: "/img/programmer-01.jpg", alt: "Perfil" }}
            imageProps={{ src: "/img/programmer-02.jpg", alt: "Capa" }}
            titleProps={{ title: "Ana Silva" }}
            tagsProps={{ tags: cardTags }}
            actionsProps={{ actions: cardActions }}
            descriptionProps={{ description: "Designer de produto em Luanda." }}
            buttonProps={{ text: "Seguir" }}
            cartButtonProps={{}}
            favoriteButtonProps={{}}
          />
        </div>
      {:else if component.id === "card-product"}
        <div class="w-full max-w-xs">
          <CardProduct
            variant="01"
            logoProps={{ src: "/img/programmer-01.jpg", alt: "Marca" }}
            imageProps={{ src: "/img/ux-01.jpg", alt: "Produto" }}
            titleProps={{ title: "Teclado Mecânico" }}
            ratingProps={{ rating: 5 }}
            priceProps={{ number: 349.9 }}
            descriptionProps={{
              description: "Switch táctil com retroiluminação RGB.",
            }}
            minFileProps={cardMinFiles}
            buttonProps={{ text: "Comprar" }}
            cartButtonProps={{}}
            favoriteButtonProps={{}}
          />
        </div>
      {:else if component.id === "card"}
        <div class="w-full max-w-xs"><Card /></div>
      {:else if component.id === "card-image"}
        <div class="w-40 overflow-hidden rounded-lg">
          <CardImage src="/img/ux-01.jpg" alt="Imagem de cartão" />
        </div>
      {:else if component.id === "card-title"}
        <CardTitle title="Título do cartão" />
      {:else if component.id === "card-description"}
        <div class="max-w-xs">
          <CardDescription
            description="Descrição breve do conteúdo do cartão."
          />
        </div>
      {:else if component.id === "card-price"}
        <CardPrice number={129.9} />
      {:else if component.id === "card-rating"}
        <CardRating rating={4} />
      {:else if component.id === "card-tags"}
        <CardTags tags={cardTags} />
      {:else if component.id === "card-logo"}
        <CardLogo src="/img/programmer-01.jpg" alt="Logótipo" />
      {:else if component.id === "card-phone"}
        <CardPhone phone="+244 900 000 000" />
      {:else if component.id === "card-video"}
        <div class="w-56 overflow-hidden rounded-lg">
          <CardVideo src="" poster="/img/ux-01.jpg" preload="none" controls />
        </div>
      {:else if component.id === "card-thumbnail-video"}
        <div class="w-56">
          <CardThumbnailVideo
            videoProps={{ src: "" }}
            imageProps={{ src: "/img/ux-01.jpg", alt: "Miniatura de vídeo" }}
            visibleButtonPlay
          />
        </div>
      {:else if component.id === "card-min-file"}
        <div class="flex gap-2">
          {#each cardMinFiles as minFile}<CardMinFile {...minFile} />{/each}
        </div>
      {:else if component.id === "card-button"}
        <CardButton text="Acção" />
      {:else if component.id === "card-cart-button"}
        <CardCartButton />
      {:else if component.id === "card-favorite-button"}
        <CardFavoriteButton />
      {:else if component.id === "card-actions-button"}
        <CardActionsButton actions={cardActions} />
      {:else if component.id === "country-popover"}
        <CountryPhonePopover flag={FlagNamed.Ao} locale="pt" />
      {/if}
    </div>

    <div class="space-y-1.5 p-4">
      <div class="flex items-center justify-between gap-3">
        <h3 class="truncate text-sm font-semibold text-zinc-100">
          {#if component.href}
            <a class="hover:text-blue-500" href={component.href}
              >{component.label ?? component.name}</a
            >
          {:else}
            {component.label ?? component.name}
          {/if}
        </h3>
        <span
          class="shrink-0 text-[10px] font-medium uppercase tracking-wide text-zinc-400"
          >{component.group}</span
        >
      </div>
      <p class="text-xs leading-5 text-zinc-400">{component.description}</p>
      {#if component.href || component.dependence}
        <div class="flex flex-wrap gap-2 pt-2">
          {#if component.href}
            <a
              href={component.href}
              class="rounded border border-blender-border px-2.5 py-1.5 text-[11px] font-medium text-zinc-300 transition-colors hover:bg-blender-hover hover:text-zinc-100"
              >Ver terminal</a
            >
          {/if}
          {#if component.dependence}
            <button
              type="button"
              class="rounded bg-blue-600 px-2.5 py-1.5 text-[11px] font-medium text-white transition-colors hover:bg-blue-500 disabled:cursor-not-allowed disabled:opacity-40"
              onclick={() => onCopy(component)}
              disabled={!$activeProject ||
                $activeProject.language === "Unknown" ||
                copyingId === component.id}
            >
              {copyingId === component.id
                ? "A copiar…"
                : "Copiar para projecto"}
            </button>
          {/if}
        </div>
      {/if}
    </div>
  </article>
{/snippet}

<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-2">
  {#each visibleComponents as component (component.id)}
    {@render componentCard(component)}
  {/each}

  {#if sharedCardComponents.length}
    <details
      class="col-span-full overflow-hidden rounded-xl border border-blender-border bg-blender-card"
    >
      <summary
        class="flex cursor-pointer list-none items-center justify-between gap-3 px-4 py-3 text-sm font-semibold text-zinc-100 transition-colors hover:bg-blender-hover"
      >
        <span>Molecules · Card · Shared</span>
        <span class="flex items-center gap-2 text-xs font-normal text-zinc-400">
          {sharedCardComponents.length} componentes
          <span aria-hidden="true">⌄</span>
        </span>
      </summary>
      <div
        class="grid grid-cols-1 gap-4 border-t border-blender-border p-4 sm:grid-cols-2 xl:grid-cols-2"
      >
        {#each sharedCardComponents as component (component.id)}
          {@render componentCard(component)}
        {/each}
      </div>
    </details>
  {/if}
</div>
