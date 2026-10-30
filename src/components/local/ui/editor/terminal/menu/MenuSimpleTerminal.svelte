<script lang="ts">
  import MenuSimple from "../../../../../../project/ui/organisms/menu/simple/ui/Menu.svelte";
  import svelteSource from "./SubmenuGridTitleMenu.svelte?raw";
  import reactSource from "./SubmenuGridTitleMenu.tsx?raw";
  import vueSource from "./SubmenuGridTitleMenu.vue?raw";
  import flutterSource from "./SubmenuGridTitleMenu.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="menu-simple">
  {#snippet preview()}
    <article
      class="min-h-[68vh] overflow-hidden rounded-xl border border-blender-border bg-blender-base"
    >
      <MenuSimple
        logo={{
          src: "/img/programmer-01.jpg",
          alt: "Negodesign",
          className: "h-9 w-9 rounded-md object-cover",
        }}
        menus={[
          { href: "/", icon: { name: "Home" }, textProps: { text: "Início" } },
          {
            label: "Plataforma",
            title: {
              title: { text: "Uma plataforma para criar com confiança" },
              description: {
                text: "Organize projectos, componentes e sistemas num só espaço, pronto para crescer com a sua equipa.",
              },
              href: "/main/project",
              link: { text: "Saiba mais" },
            },
            items: [
              {
                title: { textProps: { text: "PRINCIPAIS RECURSOS" } },
                items: [
                  {
                    href: "/main/project",
                    title: { text: "Biblioteca de componentes" },
                    description: {
                      text: "Construa interfaces consistentes com componentes reutilizáveis.",
                    },
                  },
                  {
                    href: "/main/project",
                    title: { text: "Editor visual" },
                    description: {
                      text: "Explore e ajuste componentes num espaço interactivo.",
                    },
                  },
                  {
                    href: "/main/project",
                    title: { text: "Gestão de projecto" },
                    description: {
                      text: "Mantenha recursos e decisões organizados.",
                    },
                  },
                  {
                    href: "/main/project",
                    title: { text: "Colaboração" },
                    description: {
                      text: "Partilhe ferramentas e ideias com a sua equipa.",
                    },
                  },
                ],
              },
              {
                title: { textProps: { text: "DESCOBRIR" } },
                items: [
                  {
                    href: "/main/project",
                    title: { text: "Começar" },
                    description: {
                      text: "Conheça os primeiros passos da plataforma.",
                    },
                  },
                  {
                    href: "/main/project",
                    title: { text: "Documentação" },
                    description: {
                      text: "Consulte padrões, propriedades e exemplos.",
                    },
                  },
                  {
                    href: "/main/project",
                    title: { text: "Actualizações" },
                    description: { text: "Veja as novidades mais recentes." },
                  },
                ],
              },
            ],
          },
          { href: "/main/project", textProps: { text: "Equipa" } },
        ]}
        defaultOpenSubmenu={1}
        buttonsProps={[
          { text: "Entrar", variant: "outline" },
          { text: "Criar conta", variant: "solid" },
        ]}
      />
      <div
        class="flex min-h-[60vh] flex-col items-center justify-center px-8 py-12 text-center"
      >
        <p
          class="text-xs font-semibold uppercase tracking-[0.18em] text-blue-400"
        >
          Negodesign
        </p>
        <h2 class="mt-4 max-w-xl text-3xl font-semibold text-zinc-100">
          Crie experiências digitais à sua maneira
        </h2>
        <p class="mt-3 max-w-lg text-sm leading-relaxed text-zinc-400">
          Explore o menu Plataforma e o submenu em grelha, com recursos
          agrupados.
        </p>
      </div>
    </article>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/components/local/ui/editor/terminal/menu/SubmenuGridTitleMenu.svelte"
      source={svelteSource}
    />
  {/snippet}
  {#snippet react()}<SourceCode path="src/components/local/ui/editor/terminal/menu/SubmenuGridTitleMenu.tsx" source={reactSource} />{/snippet}
  {#snippet vue()}<SourceCode path="src/components/local/ui/editor/terminal/menu/SubmenuGridTitleMenu.vue" source={vueSource} />{/snippet}
  {#snippet flutter()}<SourceCode path="src/components/local/ui/editor/terminal/menu/SubmenuGridTitleMenu.dart" source={flutterSource} />{/snippet}
</Terminal>
