<script lang="ts">
  import PageHeroSimple from "../../../../../project/ui/pages/hero/simple/ui/PageHeroSimple.svelte";
  import PageHeroDual from "../../../../../project/ui/pages/hero/dual/ui/PageHeroDual.svelte";
  import PageHeroGrid from "../../../../../project/ui/pages/hero/grid/ui/PageHeroGrid.svelte";
  import type { PageHeroGridCardProps } from "../../../../../project/ui/pages/hero/grid/data/types";
  import ComponentScreenGroup from "../shared/ComponentScreenGroup.svelte";
  import { templateScreenNavigation } from "../shared/template-screen-navigation";
  import SourceCode from "../terminal/SourceCode.svelte";
  import Terminal from "../terminal/Terminal.svelte";

  import pageHeroSimpleSvelte from "../../../../../project/ui/pages/hero/simple/ui/PageHeroSimple.svelte?raw";
  import pageHeroSimpleReact from "../../../../../project/ui/pages/hero/simple/ui/PageHeroSimple.tsx?raw";
  import pageHeroSimpleVue from "../../../../../project/ui/pages/hero/simple/ui/PageHeroSimple.vue?raw";
  import pageHeroSimpleFlutter from "../../../../../project/ui/pages/hero/simple/ui/PageHeroSimple.dart?raw";
  import pageHeroDualSvelte from "../../../../../project/ui/pages/hero/dual/ui/PageHeroDual.svelte?raw";
  import pageHeroDualReact from "../../../../../project/ui/pages/hero/dual/ui/PageHeroDual.tsx?raw";
  import pageHeroDualVue from "../../../../../project/ui/pages/hero/dual/ui/PageHeroDual.vue?raw";
  import pageHeroDualFlutter from "../../../../../project/ui/pages/hero/dual/ui/PageHeroDual.dart?raw";
  import pageHeroGridSvelte from "../../../../../project/ui/pages/hero/grid/ui/PageHeroGrid.svelte?raw";
  import pageHeroGridReact from "../../../../../project/ui/pages/hero/grid/ui/PageHeroGrid.tsx?raw";
  import pageHeroGridVue from "../../../../../project/ui/pages/hero/grid/ui/PageHeroGrid.vue?raw";
  import pageHeroGridFlutter from "../../../../../project/ui/pages/hero/grid/ui/PageHeroGrid.dart?raw";

  const group = templateScreenNavigation.find(
    (item) => item.type === "hero",
  )!;

  const heroMenuProps = {
    logo: {
      src: "/favicon.png",
      alt: "Negodesign",
      className: "size-9 rounded-full",
    },
    menus: [
      { href: "#hero", textProps: { text: "Início" } },
      { href: "#template-screen-panel", textProps: { text: "Autenticação" } },
      { href: "#template-screen-panel", textProps: { text: "Exemplos" } },
    ],
    buttonsProps: [
      {
        text: "Ver páginas",
        variant: "solid" as const,
        onClick: () =>
          document
            .getElementById("template-screen-panel")
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

<div class="space-y-5">
  <ComponentScreenGroup
    label={group.label}
    badge={group.badge}
    description={group.description}
    count={group.count}
  />

  <Terminal
    id="page-hero-simple"
    componentId="page-hero-simple"
    name="PageHeroSimple"
  >
    {#snippet preview()}
      <article
        class="flex min-w-0 flex-col overflow-hidden rounded-lg border border-blender-border bg-blender-base"
      >
        <div
          class="relative min-h-70 flex-1 overflow-hidden bg-blender-base"
        >
          <div
            class="origin-top-left scale-75"
            style="width: 133.333%; height: 133.333%;"
          >
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
          </div>
          <div
            class="pointer-events-none absolute inset-x-0 bottom-0 h-8 bg-linear-to-t from-blender-base/60 to-transparent"
          ></div>
        </div>
      </article>
    {/snippet}

    {#snippet svelte()}
      <SourceCode
        path="src/project/ui/pages/hero/simple/ui/PageHeroSimple.svelte"
        source={pageHeroSimpleSvelte}
      />
    {/snippet}
    {#snippet react()}
      <SourceCode
        path="src/project/ui/pages/hero/simple/ui/PageHeroSimple.tsx"
        source={pageHeroSimpleReact}
      />
    {/snippet}
    {#snippet vue()}
      <SourceCode
        path="src/project/ui/pages/hero/simple/ui/PageHeroSimple.vue"
        source={pageHeroSimpleVue}
      />
    {/snippet}
    {#snippet flutter()}
      <SourceCode
        path="src/project/ui/pages/hero/simple/ui/PageHeroSimple.dart"
        source={pageHeroSimpleFlutter}
      />
    {/snippet}
  </Terminal>

  <Terminal
    id="page-hero-simple-02"
    componentId="page-hero-simple"
    name="PageHeroSimple02"
  >
    {#snippet preview()}
      <article
        class="flex min-w-0 flex-col overflow-hidden rounded-lg border border-blender-border bg-blender-base"
      >
        <div
          class="relative min-h-70 flex-1 overflow-hidden bg-blender-base"
        >
          <div
            class="origin-top-left scale-75"
            style="width: 133.333%; height: 133.333%;"
          >
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
          </div>
          <div
            class="pointer-events-none absolute inset-x-0 bottom-0 h-8 bg-linear-to-t from-blender-base/60 to-transparent"
          ></div>
        </div>
      </article>
    {/snippet}

    {#snippet svelte()}
      <SourceCode
        path="src/project/ui/pages/hero/simple/ui/PageHeroSimple.svelte"
        source={pageHeroSimpleSvelte}
      />
    {/snippet}
    {#snippet react()}
      <SourceCode
        path="src/project/ui/pages/hero/simple/ui/PageHeroSimple.tsx"
        source={pageHeroSimpleReact}
      />
    {/snippet}
    {#snippet vue()}
      <SourceCode
        path="src/project/ui/pages/hero/simple/ui/PageHeroSimple.vue"
        source={pageHeroSimpleVue}
      />
    {/snippet}
    {#snippet flutter()}
      <SourceCode
        path="src/project/ui/pages/hero/simple/ui/PageHeroSimple.dart"
        source={pageHeroSimpleFlutter}
      />
    {/snippet}
  </Terminal>

  <Terminal
    id="page-hero-dual"
    componentId="page-hero-dual"
    name="PageHeroDual"
  >
    {#snippet preview()}
      <article
        class="flex min-w-0 flex-col overflow-hidden rounded-lg border border-blender-border bg-blender-base"
      >
        <div
          class="relative min-h-70 flex-1 overflow-hidden bg-blender-base"
        >
          <div
            class="origin-top-left scale-75"
            style="width: 133.333%; height: 133.333%;"
          >
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
          </div>
          <div
            class="pointer-events-none absolute inset-x-0 bottom-0 h-8 bg-linear-to-t from-blender-base/60 to-transparent"
          ></div>
        </div>
      </article>
    {/snippet}

    {#snippet svelte()}
      <SourceCode
        path="src/project/ui/pages/hero/dual/ui/PageHeroDual.svelte"
        source={pageHeroDualSvelte}
      />
    {/snippet}
    {#snippet react()}
      <SourceCode
        path="src/project/ui/pages/hero/dual/ui/PageHeroDual.tsx"
        source={pageHeroDualReact}
      />
    {/snippet}
    {#snippet vue()}
      <SourceCode
        path="src/project/ui/pages/hero/dual/ui/PageHeroDual.vue"
        source={pageHeroDualVue}
      />
    {/snippet}
    {#snippet flutter()}
      <SourceCode
        path="src/project/ui/pages/hero/dual/ui/PageHeroDual.dart"
        source={pageHeroDualFlutter}
      />
    {/snippet}
  </Terminal>

  <Terminal
    id="page-hero-grid"
    componentId="page-hero-grid"
    name="PageHeroGrid"
  >
    {#snippet preview()}
      <article
        class="flex min-w-0 flex-col overflow-hidden rounded-lg border border-blender-border bg-blender-base"
      >
        <div
          class="relative aspect-[1.05] min-h-80 max-h-170 overflow-hidden bg-blender-base"
        >
          <div
            class="origin-top-left scale-75"
            style="width: 133.333%; height: 133.333%;"
          >
            <PageHeroGrid cards={heroGridCards} />
          </div>
          <div
            class="pointer-events-none absolute inset-x-0 bottom-0 h-8 bg-linear-to-t from-blender-base/60 to-transparent"
          ></div>
        </div>
      </article>
    {/snippet}

    {#snippet svelte()}
      <SourceCode
        path="src/project/ui/pages/hero/grid/ui/PageHeroGrid.svelte"
        source={pageHeroGridSvelte}
      />
    {/snippet}
    {#snippet react()}
      <SourceCode
        path="src/project/ui/pages/hero/grid/ui/PageHeroGrid.tsx"
        source={pageHeroGridReact}
      />
    {/snippet}
    {#snippet vue()}
      <SourceCode
        path="src/project/ui/pages/hero/grid/ui/PageHeroGrid.vue"
        source={pageHeroGridVue}
      />
    {/snippet}
    {#snippet flutter()}
      <SourceCode
        path="src/project/ui/pages/hero/grid/ui/PageHeroGrid.dart"
        source={pageHeroGridFlutter}
      />
    {/snippet}
  </Terminal>
</div>
