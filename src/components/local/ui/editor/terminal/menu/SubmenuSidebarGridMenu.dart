import 'package:flutter/material.dart';
import '../../../../../../project/ui/organisms/menu/simple/ui/Menu.dart';
import '../../../../../../project/ui/organisms/menu/simple/data/types.dart';
import '../../../../../../project/ui/organisms/menu/shared/submenu/grid/sidebar/data/types.dart';
import '../../../../../../project/ui/organisms/menu/shared/submenu/simple/title/data/types.dart';
import '../../../../../../project/ui/organisms/menu/shared/menu/menu-text/data/types.dart';
import '../../../../../../project/ui/organisms/menu/shared/menu/menu-title/data/types.dart';
import '../../../../../../project/ui/atoms/text/data/types.dart';
import '../../../../../../project/ui/atoms/image/data/types.dart';
import '../../../../../../project/ui/atoms/button/variant/default/type/types.dart';
import '../../../../../../project/ui/atoms/icon/data/types.dart';

const solutionsMenu = SubmenuSidebarGridProps(type: 'sidebar-grid', title: MenuTitleProps(title: TextProps(Text: 'Soluções para cada etapa do seu projecto'), description: TextProps(Text: 'Explore áreas de trabalho e encontre recursos para a sua equipa.'), href: '/main/project', link: TextProps(Text: 'Ver todas as soluções')), defaultSelected: 0, items: [
  SubmenuTitleProps(title: MenuTextProps(textProps: TextProps(Text: 'Design de produto')), items: [MenuTitleProps(title: TextProps(Text: 'Biblioteca de componentes'), description: TextProps(Text: 'Componentes reutilizáveis para interfaces consistentes.'), href: '/main/project'), MenuTitleProps(title: TextProps(Text: 'Sistemas de design'), description: TextProps(Text: 'Organize padrões e tokens para toda a equipa.'), href: '/main/project')]),
  SubmenuTitleProps(title: MenuTextProps(textProps: TextProps(Text: 'Desenvolvimento')), items: [MenuTitleProps(title: TextProps(Text: 'Aplicações web'), description: TextProps(Text: 'Desenvolva produtos modernos e adaptáveis.'), href: '/main/project'), MenuTitleProps(title: TextProps(Text: 'Integrações'), description: TextProps(Text: 'Ligue ferramentas e serviços do seu projecto.'), href: '/main/project')]),
]);

class SubmenuSidebarGridMenu extends StatelessWidget {
  const SubmenuSidebarGridMenu({super.key});
  @override
  Widget build(BuildContext context) => Menu(MenuProps(logo: const ImageProps(src: '/img/programmer-02.jpg', alt: 'Negodesign'), menus: const [MenuTextProps(href: '/', icon: IconProps(name: 'Home'), textProps: TextProps(Text: 'Início')), solutionsMenu, MenuTextProps(href: '/main/project', textProps: TextProps(Text: 'Projectos'))], buttonsProps: const [ButtonProps(text: 'Entrar', variant: 'outline'), ButtonProps(text: 'Começar', variant: 'solid')], defaultOpenSubmenu: 1));
}
