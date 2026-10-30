<script lang="ts">
  import Menu from "../../../../../../project/ui/organisms/menu/simple/ui/Menu.svelte";
  import type { SubmenuGridTitleProps } from "../../../../../../project/ui/organisms/menu/shared/submenu/grid/title/data/types";

  const platformMenu: SubmenuGridTitleProps = {
    label: "Plataforma",
    title: {
      title: { text: "Uma plataforma para criar com confiança" },
      description: { text: "Organize projectos, componentes e sistemas num só espaço, pronto para crescer com a sua equipa." },
      href: "/main/project",
      link: { text: "Saiba mais" },
    },
    items: [
      {
        title: { text: "PRINCIPAIS RECURSOS" },
        items: [
          { href: "/main/project", title: { text: "Biblioteca de componentes" }, description: { text: "Construa interfaces consistentes com componentes reutilizáveis." } },
          { href: "/main/project", title: { text: "Editor visual" }, description: { text: "Explore e ajuste componentes num espaço interactivo." } },
          { href: "/main/project", title: { text: "Gestão de projecto" }, description: { text: "Mantenha recursos e decisões organizados." } },
          { href: "/main/project", title: { text: "Colaboração" }, description: { text: "Partilhe ferramentas e ideias com a sua equipa." } },
        ],
      },
      {
        title: { text: "DESCOBRIR" },
        items: [
          { href: "/main/project", title: { text: "Começar" }, description: { text: "Conheça os primeiros passos da plataforma." } },
          { href: "/main/project", title: { text: "Documentação" }, description: { text: "Consulte padrões, propriedades e exemplos." } },
          { href: "/main/project", title: { text: "Actualizações" }, description: { text: "Veja as novidades mais recentes." } },
        ],
      },
    ],
  };
</script>

<Menu
  logo={{ src: "/img/programmer-01.jpg", alt: "Negodesign", className: "h-9 w-9 rounded-md object-cover" }}
  menus={[
    { href: "/", icon: { name: "Home" }, textProps: { text: "Início" } },
    platformMenu,
    { href: "/main/project", textProps: { text: "Equipa" } },
  ]}
  buttonsProps={[{ text: "Entrar", variant: "outline" }, { text: "Criar conta", variant: "solid" }]}
  defaultOpenSubmenu={1}
/>
