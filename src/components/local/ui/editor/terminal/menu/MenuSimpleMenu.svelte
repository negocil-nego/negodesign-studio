<script lang="ts">
  import Menu from "../../../../../../project/ui/organisms/menu/simple/ui/Menu.svelte";
  import type { ButtonProps } from "../../../../../../project/ui/atoms/button/variant/default/type/types";

  const menus = [
    { href: "/", icon: { name: "Home" }, textProps: { text: "Início" } },
    { href: "/projectos", textProps: { text: "Projectos" } },
    { href: "/equipa", textProps: { text: "Equipa" } },
  ];
  const buttonsProps: ButtonProps[] = [
    { text: "Entrar", variant: "outline" },
    { text: "Criar conta", variant: "solid" },
  ];
</script>

<Menu
  logo={{ src: "/img/programmer-01.jpg", alt: "Negodesign", className: "h-9 w-9 rounded-md object-cover" }}
  {menus}
  {buttonsProps}
/>
