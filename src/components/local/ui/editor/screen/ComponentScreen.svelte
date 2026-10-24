<script lang="ts">
  // ── Atoms · Input ──────────────────────────────────────────────────────────
  import Input from "../../../../../project/ui/atoms/input/core/ui/Input.svelte";
  import InputText from "../../../../../project/ui/atoms/input/children/text/ui/InputText.svelte";
  import InputEmail from "../../../../../project/ui/atoms/input/children/email/ui/InputEmail.svelte";
  import InputPhone from "../../../../../project/ui/atoms/input/children/phone/ui/InputPhone.svelte";
  import InputPhoneFlag from "../../../../../project/ui/atoms/input/children/phone/ui/InputPhoneFlag.svelte";
  import InputPassword from "../../../../../project/ui/atoms/input/children/password/ui/InputPassword.svelte";
  import InputDate from "../../../../../project/ui/atoms/input/children/date/ui/InputDate.svelte";
  import InputTime from "../../../../../project/ui/atoms/input/children/time/ui/InputTime.svelte";
  import InputNumber from "../../../../../project/ui/atoms/input/children/number/ui/InputNumber.svelte";
  import InputAge from "../../../../../project/ui/atoms/input/children/age/ui/InputAge.svelte";
  import InputLabel from "../../../../../project/ui/atoms/input/children/label/ui/InputLabel.svelte";

  // ── Atoms · Control ────────────────────────────────────────────────────────
  import Select from "../../../../../project/ui/atoms/select/ui/Select.svelte";
  import Textarea from "../../../../../project/ui/atoms/textarea/ui/Textarea.svelte";
  import Toggle from "../../../../../project/ui/atoms/toggle/ui/Toggle.svelte";
  import Radio from "../../../../../project/ui/atoms/radio/ui/Radio.svelte";
  import Checkbox from "../../../../../project/ui/atoms/checkbox/ui/Checkbox.svelte";

  // ── Molecules ──────────────────────────────────────────────────────────────
  import CountryPhonePopover from "../../../../../project/ui/molecules/popover/ui/CountryPhonePopover.svelte";
  import { FlagNamed } from "../../../../../hooks/flag.svelte";

  // ── Molecules · Card ───────────────────────────────────────────────────────
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

  import type { CardTag } from "../../../../../project/ui/molecules/card/shared/card-tags/data/types";
  import type { CardActionButton } from "../../../../../project/ui/molecules/card/shared/card-actions-button/data/types";
  import type { CardMinFileProps } from "../../../../../project/ui/molecules/card/shared/card-min-file/data/types";

  import { componentCatalog, type ComponentDefinition } from "../../../data/components";
  import { activeProject } from "../../../data/projects";
  import { copyComponentToActiveProject } from "../../../data/component-copy";
    import Label from "../../../../../project/ui/atoms/label/ui/label.svelte";

  // ─── Types & helpers ────────────────────────────────────────────────────────

  type Group = {
    label: string;
    badge: string;
    description: string;
    ids: string[];
  };

  /** Logical grouping for display – keeps Input atoms and Control atoms separate. */
  const screenGroups: Group[] = [
    {
      label: "Atoms · Input",
      badge: "Atom",
      description: "Campos de entrada de texto e dados estruturados.",
      ids: [
        "input",
        "input-text",
        "input-email",
        "input-phone",
        "input-phone-flag",
        "input-label",
        "input-password",
        "input-date",
        "input-time",
        "input-number",
        "input-age",
      ],
    },
    {
      label: "Atoms · Control",
      badge: "Control",
      description: "Controlos de selecção e alternância.",
      ids: ["select", "textarea", "toggle", "radio", "checkbox"],
    },
    {
      label: "Typography",
      badge: "Atom",
      description: "Elementos de texto e rótulo.",
      ids: ["label"],
    },
    {
      label: "Molecules",
      badge: "Molecule",
      description: "Componentes compostos por múltiplos átomos.",
      ids: ["country-popover"],
    },
    {
      label: "Molecules · Card",
      badge: "Molecule",
      description: "Cartões compostos por átomos e sub-cartões reutilizáveis.",
      ids: [
        "card-media",
        "card-profile",
        "card-product",
        "card",
        "card-image",
        "card-title",
        "card-description",
        "card-price",
        "card-rating",
        "card-tags",
        "card-logo",
        "card-phone",
        "card-video",
        "card-thumbnail-video",
        "card-min-file",
        "card-button",
        "card-cart-button",
        "card-favorite-button",
        "card-actions-button",
      ],
    },
  ];

  const catalogById = Object.fromEntries(componentCatalog.map((c) => [c.id, c]));

  const components = componentCatalog;
  let copyingId = $state<string | null>(null);
  let copyMessage = $state("");

  // ── Demo data for control previews ──────────────────────────────────────────
  const selectOptions = [
    { value: "ao", label: "Angola" },
    { value: "pt", label: "Portugal" },
    { value: "br", label: "Brasil" },
  ];

  const radioOptions = [
    { value: "svelte", label: "Svelte" },
    { value: "react", label: "React" },
    { value: "vue", label: "Vue" },
  ];

  // ── Demo data for card previews ────────────────────────────────────────────
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

  async function copyComponent(component: (typeof components)[number]) {
    if (!component.dependence) return;
    copyingId = component.id;
    copyMessage = "";
    try {
      const result = await copyComponentToActiveProject(component.id);
      copyMessage = `${component.label ?? component.name}: ${result.copiedFiles} ficheiro(s) e ${result.exportedComponents} componente(s) disponíveis em ${result.project.name}/negodesign.`;
    } catch (error) {
      copyMessage = error instanceof Error ? error.message : String(error);
    } finally {
      copyingId = null;
    }
  }
</script>

<main class="space-y-8">
  <!-- ── Header ──────────────────────────────────────────────────────────── -->
  <header class="flex items-end justify-between gap-4">
    <div>
      <p class="text-xs font-semibold uppercase tracking-[0.16em] text-blue-500">
        Project UI
      </p>
      <h1 class="mt-1 text-2xl font-semibold text-zinc-100">Componentes</h1>
      <p class="mt-1 text-sm text-zinc-400">
        Pré-visualização dos componentes existentes neste projecto.
      </p>
    </div>
    <span
      class="shrink-0 rounded-full border border-blender-border bg-blender-card px-3 py-1 text-xs font-medium text-zinc-400"
    >
      {components.length} componentes
    </span>
  </header>

  <!-- ── Active project bar ──────────────────────────────────────────────── -->
  <section
    class="flex flex-wrap items-center justify-between gap-3 rounded-lg border border-blender-border bg-blender-card px-4 py-3"
  >
    <div class="min-w-0">
      <p class="text-[10px] font-semibold uppercase tracking-wider text-zinc-500">
        Linguagem do projecto activo
      </p>
      {#if $activeProject}
        <p class="mt-1 truncate text-sm font-semibold text-zinc-100">
          {$activeProject.language}
          <span class="font-normal text-zinc-400">· {$activeProject.name}</span>
        </p>
      {:else}
        <p class="mt-1 text-sm text-zinc-400">
          Escolha um projecto no Inspector para activar a cópia de componentes.
        </p>
      {/if}
    </div>
    {#if $activeProject}
      <span
        class="rounded-full border border-blue-500/30 bg-blue-500/10 px-3 py-1 text-xs font-medium text-blue-400"
        >{$activeProject.language}</span
      >
    {/if}
  </section>

  {#if copyMessage}
    <p class="text-xs text-zinc-400" role="status">{copyMessage}</p>
  {/if}

  <!-- ── Component groups ────────────────────────────────────────────────── -->
  {#each screenGroups as group}
    {@const groupItems = group.ids.map((id) => catalogById[id]).filter(Boolean)}
    {#if groupItems.length}
      <section class="space-y-4">
        <!-- Group header -->
        <div class="flex items-end gap-3 border-b border-blender-border pb-2">
          <div class="min-w-0 flex-1">
            <p class="text-[10px] font-semibold uppercase tracking-wider text-zinc-500">
              {group.badge}
            </p>
            <h2 class="text-base font-semibold text-zinc-100">{group.label}</h2>
            <p class="mt-0.5 text-xs text-zinc-500">{group.description}</p>
          </div>
          <span
            class="shrink-0 rounded-full border border-blender-border bg-blender-base px-2.5 py-0.5 text-[10px] font-medium text-zinc-500"
            >{groupItems.length}</span
          >
        </div>

        <!-- Cards grid -->
        <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-3">
          {#each groupItems as component (component.id)}
            <article
              class="overflow-hidden rounded-xl border border-blender-border bg-blender-card shadow-sm transition-shadow hover:shadow-md"
            >
              <!-- Preview area -->
              <div class="flex min-h-36 items-center justify-center bg-blender-base p-5">
                {#if component.id === "input"}
                  <div class="w-full max-w-xs">
                    <Input type="text" placeholder="Introduza um valor" />
                  </div>
                {:else if component.id === "input-text"}
                  <div class="w-full max-w-xs">
                    <InputText
                      inputProps={{ id: "gallery-text", type: "text", placeholder: "Introduza um valor" }}
                    />
                  </div>
                {:else if component.id === "input-email"}
                  <div class="w-full max-w-xs">
                    <InputEmail
                      inputProps={{ id: "gallery-email", type: "email", placeholder: "nome@exemplo.com" }}
                    />
                  </div>
                {:else if component.id === "input-phone"}
                  <div class="w-full max-w-xs">
                    <InputPhone
                      inputProps={{ id: "gallery-phone", type: "tel", placeholder: "+244 900 000 000" }}
                    />
                  </div>
                {:else if component.id === "input-phone-flag"}
                  <div class="w-full max-w-xs">
                    <InputPhoneFlag
                      flag={[FlagNamed.Ao, FlagNamed.Ng]}
                      inputProps={{ id: "gallery-phone-flag", type: "tel", placeholder: "900 000 000" }}
                    />
                  </div>
                {:else if component.id === "input-password"}
                  <div class="w-full max-w-xs">
                    <InputPassword
                      inputProps={{ id: "gallery-password", type: "password", placeholder: "Palavra-passe" }}
                    />
                  </div>
                {:else if component.id === "input-date"}
                  <div class="w-full max-w-xs">
                    <InputDate inputProps={{ id: "gallery-date", type: "date" }} />
                  </div>
                {:else if component.id === "input-time"}
                  <div class="w-full max-w-xs">
                    <InputTime inputProps={{ id: "gallery-time", type: "time" }} />
                  </div>
                {:else if component.id === "input-number"}
                  <div class="w-full max-w-xs">
                    <InputNumber
                      inputProps={{ id: "gallery-number", type: "number", placeholder: "0" }}
                    />
                  </div>
                {:else if component.id === "input-age"}
                  <div class="w-full max-w-xs">
                    <InputAge
                      labelProps={{ label: "Idade", for: "gallery-age" }}
                      inputProps={{ id: "gallery-age", type: "number", placeholder: "18" }}
                    />
                  </div>
                {:else if component.id === "input-label"}
                  <div class="w-full max-w-xs">
                    <InputLabel
                      labelProps={{ label: "Nome", for: "gallery-input-label" }}
                      inputProps={{ id: "gallery-input-label", type: "text", placeholder: "O seu nome" }}
                    />
                  </div>
                {:else if component.id === "label"}
                  <div class="w-full max-w-xs space-y-2">
                    <Label label="Nome" for="gallery-label" />
                    <input
                      id="gallery-label"
                      type="text"
                      placeholder="O seu nome"
                      class="w-full rounded-md border border-blender-border bg-blender-card px-3 py-2 text-sm text-zinc-200 placeholder:text-zinc-500"
                    />
                  </div>
                {:else if component.id === "select"}
                  <div class="w-full max-w-xs">
                    <Select
                      id="gallery-select"
                      options={selectOptions}
                      placeholder="Seleccione um país…"
                    />
                  </div>
                {:else if component.id === "textarea"}
                  <div class="w-full max-w-xs">
                    <Textarea
                      id="gallery-textarea"
                      placeholder="Escreva aqui o seu texto…"
                      rows={3}
                    />
                  </div>
                {:else if component.id === "toggle"}
                  <div class="flex flex-col gap-3">
                    <Toggle id="gallery-toggle-off" label="Desactivado" />
                    <Toggle id="gallery-toggle-on" checked={true} label="Activado" />
                  </div>
                {:else if component.id === "radio"}
                  <Radio name="gallery-radio" options={radioOptions} value="svelte" />
                {:else if component.id === "checkbox"}
                  <div class="flex flex-col gap-3">
                    <Checkbox id="gallery-checkbox-off" label="Não seleccionado" />
                    <Checkbox id="gallery-checkbox-on" checked={true} label="Seleccionado" />
                  </div>
                {:else if component.id === "card-media"}
                  <div class="w-full max-w-xs">
                    <CardMedia
                      variant="01"
                      logoProps={{ src: "/img/programmer-01.jpg", alt: "Marca" }}
                      titleProps={{ title: "Auscultadores Pro" }}
                      ratingProps={{ rating: 4 }}
                      priceProps={{ number: 129.9 }}
                      descriptionProps={{ description: "Som imersivo com cancelamento de ruído." }}
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
                      descriptionProps={{ description: "Switch táctil com retroiluminação RGB." }}
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
                    <CardDescription description="Descrição breve do conteúdo do cartão." />
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
                    {#each cardMinFiles as minFile}
                      <CardMinFile {...minFile} />
                    {/each}
                  </div>
                {:else if component.id === "card-button"}
                  <CardButton text="Acção" />
                {:else if component.id === "card-cart-button"}
                  <CardCartButton />
                {:else if component.id === "card-favorite-button"}
                  <CardFavoriteButton />
                {:else if component.id === "card-actions-button"}
                  <CardActionsButton actions={cardActions} />
                {:else}
                  <CountryPhonePopover flag={FlagNamed.Ao} locale="pt" />
                {/if}
              </div>

              <!-- Card footer -->
              <div class="space-y-1.5 p-4">
                <div class="flex items-center justify-between gap-3">
                  <h3 class="truncate text-sm font-semibold text-zinc-100">
                    {#if component.href}
                      <a class="hover:text-blue-500" href={component.href}>
                        {component.label ?? component.name}
                      </a>
                    {:else}
                      {component.label ?? component.name}
                    {/if}
                  </h3>
                  <span
                    class="shrink-0 text-[10px] font-medium uppercase tracking-wide text-zinc-400"
                    >{component.group}</span
                  >
                </div>
                <p class="text-xs leading-5 text-zinc-400">
                  {component.description}
                </p>
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
                        onclick={() => copyComponent(component)}
                        disabled={!$activeProject ||
                          $activeProject.language === "Unknown" ||
                          copyingId === component.id}
                        >{copyingId === component.id
                          ? "A copiar…"
                          : "Copiar para projecto"}</button
                      >
                    {/if}
                  </div>
                {/if}
              </div>
            </article>
          {/each}
        </div>
      </section>
    {/if}
  {/each}
</main>
