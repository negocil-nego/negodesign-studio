<script lang="ts">
  import Menu from "../../../../../../project/ui/organisms/menu/simple/ui/Menu.svelte";
  import type { SubmenuSidebarGridProps } from "../../../../../../project/ui/organisms/menu/shared/submenu/grid/sidebar/data/types";

  const solutionsMenu: SubmenuSidebarGridProps = {
    type: "sidebar-grid",
    title: {
      title: { text: "Plataforma" },
      description: {
        text: "Explore áreas de trabalho, encontre recursos e escolha a solução adequada para a sua equipa.",
      },
      href: "/main/project",
      link: { text: "Ver todas as soluções" },
    },
    defaultSelected: 0,
    items: [
      {
        title: { textProps: { text: "Design de produto" } },
        items: [
          {
            href: "/main/project",
            title: { text: "Biblioteca de componentes" },
            description: {
              text: "Componentes reutilizáveis para interfaces consistentes.",
            },
          },
          {
            href: "/main/project",
            title: { text: "Sistemas de design" },
            description: {
              text: "Organize padrões e tokens para toda a equipa.",
            },
          },
          {
            href: "/main/project",
            title: { text: "Prototipagem" },
            description: {
              text: "Valide fluxos e ideias antes de implementar.",
            },
          },
          {
            href: "/main/project",
            title: { text: "Acessibilidade" },
            description: {
              text: "Crie experiências mais claras para todas as pessoas.",
            },
          },
        ],
      },
      {
        title: { textProps: { text: "Desenvolvimento" } },
        items: [
          {
            href: "/main/project",
            title: { text: "Aplicações web" },
            description: { text: "Desenvolva produtos modernos e adaptáveis." },
          },
          {
            href: "/main/project",
            title: { text: "Integrações" },
            description: {
              text: "Ligue ferramentas e serviços do seu projecto.",
            },
          },
          {
            href: "/main/project",
            title: { text: "Documentação" },
            description: { text: "Consulte guias, exemplos e especificações." },
          },
        ],
      },
      {
        title: { textProps: { text: "Gestão de equipa" } },
        items: [
          {
            href: "/main/project",
            title: { text: "Projectos" },
            description: { text: "Mantenha tarefas e entregas organizadas." },
          },
          {
            href: "/main/project",
            title: { text: "Colaboração" },
            description: { text: "Partilhe o trabalho e alinhe decisões." },
          },
          {
            href: "/main/project",
            title: { text: "Relatórios" },
            description: { text: "Acompanhe progresso e resultados." },
          },
        ],
      },
    ],
  };
</script>

<Menu
  logo={{
    src: "/img/programmer-02.jpg",
    alt: "Negodesign",
    className: "h-9 w-9 rounded-md object-cover",
  }}
  menus={[
    { href: "/", icon: { name: "Home" }, textProps: { text: "Início" } },
    solutionsMenu,
    { href: "/main/project", textProps: { text: "Projectos" } },
  ]}
  buttonsProps={[
    { text: "Entrar", variant: "outline" },
    { text: "Começar", variant: "solid" },
  ]}
  defaultOpenSubmenu={1}
/>
