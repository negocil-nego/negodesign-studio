<script lang="ts">
  import PageHeroSimple from "../../../../project/ui/pages/hero/simple/ui/PageHeroSimple.svelte";
  import PageHeroDual from "../../../../project/ui/pages/hero/dual/ui/PageHeroDual.svelte";
  import PageHeroGrid from "../../../../project/ui/pages/hero/grid/ui/PageHeroGrid.svelte";
  import type { PageHeroGridCardProps } from "../../../../project/ui/pages/hero/grid/data/types";
  import PageLogin from "../../../../project/ui/pages/login/simple/ui/PageLogin.svelte";
  import PageLoginDual from "../../../../project/ui/pages/login/dual-section/ui/PageLoginDual.svelte";
  import PageRegister from "../../../../project/ui/pages/register/simple/ui/PageRegister.svelte";
  import PageRegisterDual from "../../../../project/ui/pages/register/dual-section/ui/PageRegisterDual.svelte";
  import PageOtp from "../../../../project/ui/pages/otp/simple/ui/PageOtp.svelte";
  import PageOtpDual from "../../../../project/ui/pages/otp/dual-section/ui/PageOtpDual.svelte";

  const groups = [
    {
      id: "hero",
      name: "Hero",
      pages: [
        { name: "PageHeroSimple", component: PageHeroSimple },
        { name: "PageHeroSimple02", component: PageHeroSimple },
        { name: "PageHeroDual", component: PageHeroDual },
        { name: "PageHeroGrid", component: PageHeroGrid },
      ],
    },
    {
      id: "authentication",
      name: "Autenticação",
      pages: [
        { name: "PageLogin", component: PageLogin },
        { name: "PageLoginDual", component: PageLoginDual },
        { name: "PageRegister", component: PageRegister },
        { name: "PageRegisterDual", component: PageRegisterDual },
        { name: "PageOtp", component: PageOtp },
        { name: "PageOtpDual", component: PageOtpDual },
      ],
    },
  ];
  const totalPages = groups.reduce(
    (total, group) => total + group.pages.length,
    0,
  );
  let activeGroup = $state("hero");

  const heroMenuProps = {
    logo: {
      src: "/favicon.png",
      alt: "Negodesign",
      className: "size-9 rounded-full",
    },
    menus: [
      { href: "#hero", textProps: { text: "Início" } },
      { href: "#authentication", textProps: { text: "Autenticação" } },
      { href: "#authentication", textProps: { text: "Exemplos" } },
    ],
    buttonsProps: [
      {
        text: "Ver páginas",
        variant: "solid" as const,
        onClick: () =>
          document
            .getElementById("authentication")
            ?.scrollIntoView({ behavior: "smooth" }),
      },
    ],
  };

  const heroGridCards: PageHeroGridCardProps[] = [
    {
      id: "feature",
      colSpan: 5,
      rowSpan: 7,
      backgroundColor: "#2b292e",
      imageUrl: "/img/ux-01.jpg",
      imageAlt: "Equipe trabalhando em uma nova experiência digital",
      imageMode: "cover",
      showOverlay: true,
      sectionCardProps: {
        contentPosition: "start",
        titleProps: { text: "CERKO / INTELLIGENCE" },
        subtitleProps: { text: "Intelligence behind every decision." },
        descriptionProps: {
          text: "A clearer view of your work, your people and what comes next.",
        },
      },
    },
    {
      id: "campaign",
      colSpan: 7,
      rowSpan: 3,
      backgroundColor: "#f1bd50",
      sectionCardProps: {
        tone: "light",
        contentPosition: "between",
        titleProps: { text: "CERKO · AI FOR WORK" },
        subtitleProps: { text: "Designed for modern workflows." },
        descriptionProps: {
          text: "Less busywork. Better decisions. More room to do your best work.",
        },
        buttonsProps: [
          { text: "Try Cerko Free", variant: "outline" },
        ],
      },
    },
    {
      id: "insights",
      colSpan: 3,
      rowSpan: 2,
      backgroundColor: "#11131c",
      imageUrl: "/img/programmer-01.jpg",
      imageAlt: "Painel digital com dados e informações",
      imageMode: "cover",
      showOverlay: true,
      sectionCardProps: {
        subtitleProps: { text: "Find the signal in every workflow." },
      },
    },
    {
      id: "teamwork",
      colSpan: 2,
      rowSpan: 2,
      backgroundColor: "#f3e9db",
      imageUrl: "/img/8233822_career_businesswoman_coin_finance.png",
      imageAlt: "Ilustração de uma equipe definindo objetivos em conjunto",
      imageMode: "contain",
    },
    {
      id: "decisions",
      colSpan: 2,
      rowSpan: 2,
      backgroundColor: "#25232b",
      imageUrl: "/img/8233822_career_businesswoman_coin_finance.png",
      imageAlt: "Ilustração de uma profissional com uma moeda",
      imageMode: "contain",
    },
    {
      id: "brand",
      colSpan: 4,
      rowSpan: 2,
      backgroundColor: "#f5e6ca",
      sectionCardProps: {
        tone: "light",
        contentPosition: "center",
        titleProps: { text: "CERKO" },
        subtitleProps: { text: "Make the next move count." },
      },
    },
    {
      id: "people",
      colSpan: 3,
      rowSpan: 2,
      backgroundColor: "#d2a878",
      imageUrl: "/img/8233822_career_businesswoman_coin_finance.png",
      imageAlt: "Equipe colaborando em uma sessão de ideias",
      imageMode: "contain",
    },
  ];
</script>

<div class="min-h-full w-full bg-blender-base p-4 md:p-6 xl:p-8">
  <div class="mx-auto flex w-full max-w-[1800px] flex-col gap-7">
    <header
      class="flex flex-wrap items-end justify-between gap-4 border-b border-blender-border pb-5"
    >
      <div>
        <p
          class="text-xs font-semibold uppercase tracking-[0.18em] text-blue-500"
        >
          Frontend <span class="px-1 text-zinc-500">/</span> Layouts
        </p>
        <h1
          class="mt-2 text-3xl font-semibold tracking-tight text-zinc-800 dark:text-zinc-100"
        >
          Templates
        </h1>
        <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-400">
          Explore páginas de destaque e fluxos de autenticação prontos para
          adaptar.
        </p>
      </div>
      <span
        class="rounded-full border border-blender-border bg-blender-card px-3 py-1.5 text-xs text-zinc-500 dark:text-zinc-400"
        >{totalPages} páginas disponíveis</span
      >
    </header>

    <nav class="flex flex-wrap gap-2" aria-label="Grupos de páginas">
      {#each groups as group (group.id)}
        <button
          type="button"
          aria-pressed={activeGroup === group.id}
          onclick={() => (activeGroup = group.id)}
          class={`inline-flex items-center gap-2 rounded-full border px-3.5 py-2 text-sm transition ${
            activeGroup === group.id
              ? "border-blue-500/50 bg-blue-500/10 text-blue-500"
              : "border-blender-border bg-blender-card text-zinc-600 hover:border-blue-500/50 hover:text-blue-500 dark:text-zinc-300"
          }`}
        >
          {group.name}
          <span
            class="rounded-full bg-blue-500/10 px-2 py-0.5 text-xs font-semibold text-blue-500"
            >{group.pages.length}</span
          >
        </button>
      {/each}
    </nav>

    {#each groups.filter((group) => group.id === activeGroup) as group (group.name)}
      <section
        id={group.id}
        class="scroll-mt-8 space-y-4"
        aria-labelledby={`group-${group.id}`}
      >
        <div class="flex items-center gap-3">
          <h2
            id={`group-${group.id}`}
            class="text-xl font-semibold text-zinc-800 dark:text-zinc-100"
          >
            {group.name}
          </h2>
          <span class="h-px flex-1 bg-blender-border"></span>
          <span
            class="rounded-full border border-blender-border px-2.5 py-1 text-xs text-zinc-500"
            >{group.pages.length} páginas</span
          >
        </div>

        <div class="grid grid-cols-1 gap-5">
          {#each group.pages as page (page.name)}
            {@const PreviewComponent = page.component}
            <article
              class="group flex min-w-0 flex-col overflow-hidden rounded-xl border border-blender-border bg-blender-card shadow-sm transition duration-200 hover:-translate-y-0.5 hover:border-blue-500/60 hover:shadow-lg"
            >
              <div
                class={`relative overflow-hidden bg-blender-base ${
                  page.name === "PageHeroGrid"
                    ? "aspect-[1.05] min-h-80 max-h-170"
                    : group.id === "hero"
                      ? "min-h-70 flex-1"
                      : "aspect-[1.55] min-h-70 max-h-115"
                }`}
              >
                <div
                  class="origin-top-left scale-75"
                  style="width: 133.333%; height: 133.333%;"
                >
                  {#if page.name === "PageHeroSimple"}
                    <PageHeroSimple
                      menuProps={heroMenuProps}
                      sectionHeroProps={{
                        backgroundMediaUrl: "/img/programmer-02.jpg",
                        backgroundColor: "#10212b",
                        textSectionProps: {
                          titleProps: {
                            text: "Experiências digitais, feitas para crescer",
                          },
                          subtitleProps: {
                            text: "Transforme boas ideias em produtos extraordinários.",
                          },
                          descriptionProps: {
                            text: "Uma base flexível para criar interfaces consistentes, acelerar equipas e levar cada projecto mais longe.",
                          },
                        },
                      }}
                    />
                  {:else if page.name === "PageHeroSimple02"}
                    <PageHeroSimple
                      menuProps={{
                        style: "ghost",
                        logo: {
                          src: "/favicon.png",
                          alt: "Adventure",
                          className: "size-9 rounded-full",
                        },
                        logoText: "Adventure",
                        logoSubtitle: "by Negodesign",
                        menus: [
                          { href: "#home", textProps: { text: "Home" } },
                          { href: "#about", textProps: { text: "About" } },
                          { href: "#destination", textProps: { text: "Destination" } },
                          { href: "#features", textProps: { text: "Features" } },
                          { href: "#pricing", textProps: { text: "Pricing" } },
                        ],
                        buttonsProps: [
                          { text: "Sign Up", variant: "outline" as const },
                        ],
                      }}
                      sectionHeroProps={{
                        style: "light",
                        backgroundColor: "#eef5f6",
                        textSectionProps: {
                          layout: "light",
                          subtitleProps: {
                            text: "Planning Your Next Virtual Journey",
                          },
                          descriptionProps: {
                            text: "Getting more focus in a minutes with a simple movement through virtual vibes",
                          },
                          buttonsProps: [
                            { text: "Explore Now", variant: "solid" as const },
                          ],
                          imageUrl:
                            "/img/8233822_career_businesswoman_coin_finance.png",
                          imageAlt:
                            "Família de várias gerações a dançar e a divertir-se em conjunto",
                        },
                      }}
                    />
                  {:else if page.name === "PageHeroDual"}
                    <PageHeroDual
                      menuProps={heroMenuProps}
                      leftSectionHereProps={{
                        textSectionProps: {
                          layout: "dual",
                          titleProps: { text: "AI para o seu negócio" },
                          subtitleProps: {
                            text: "E-Commerce reinventado com inteligência artificial",
                          },
                          descriptionProps: {
                            text: "Venda mais, ofereça uma experiência melhor e automatize operações com tecnologia de ponta.",
                          },
                        },
                      }}
                      rightSectionHereProps={{
                        textSectionProps: {
                          layout: "dual",
                          imageUrl:
                            "/img/8233822_career_businesswoman_coin_finance.png",
                          imageAlt:
                            "Demonstração visual de uma experiência digital com inteligência artificial",
                        },
                      }}
                    />
                  {:else if page.name === "PageHeroGrid"}
                    <PageHeroGrid cards={heroGridCards} />
                  {:else}
                    <PreviewComponent />
                  {/if}
                </div>
                <div
                  class="pointer-events-none absolute inset-x-0 bottom-0 h-8 bg-linear-to-t from-blender-base/60 to-transparent"
                ></div>
              </div>
              <div
                class="flex items-center justify-between gap-3 border-t border-blender-border px-4 py-3.5"
              >
                <div class="min-w-0">
                  <h3
                    class="truncate font-medium text-zinc-800 dark:text-zinc-100"
                  >
                    {page.name}
                  </h3>
                  <p class="mt-0.5 text-xs text-zinc-500">
                    {group.name} · Preview
                  </p>
                </div>
                <span
                  class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full border border-blender-border text-zinc-500 transition group-hover:border-blue-500/50 group-hover:text-blue-500"
                  aria-hidden="true">↗</span
                >
              </div>
            </article>
          {/each}
        </div>
      </section>
    {/each}
  </div>
</div>
