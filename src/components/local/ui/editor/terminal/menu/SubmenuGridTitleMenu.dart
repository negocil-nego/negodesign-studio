import 'package:flutter/material.dart';
import '../../../../../../project/ui/organisms/menu/simple/ui/Menu.dart';
import '../../../../../../project/ui/organisms/menu/simple/data/types.dart';
import '../../../../../../project/ui/organisms/menu/shared/submenu/grid/title/data/types.dart';
import '../../../../../../project/ui/organisms/menu/shared/submenu/simple/title/data/types.dart';
import '../../../../../../project/ui/organisms/menu/shared/menu/menu-text/data/types.dart';
import '../../../../../../project/ui/organisms/menu/shared/menu/menu-title/data/types.dart';
import '../../../../../../project/ui/atoms/text/data/types.dart';
import '../../../../../../project/ui/atoms/image/data/types.dart';
import '../../../../../../project/ui/atoms/button/variant/default/type/types.dart';
import '../../../../../../project/ui/atoms/icon/data/types.dart';

const platformMenu = SubmenuGridTitleProps(label: 'Plataforma', title: MenuTitleProps(title: TextProps(Text: 'Uma plataforma para criar com confiança'), description: TextProps(Text: 'Organize projectos, componentes e sistemas num só espaço.'), href: '/main/project', link: TextProps(Text: 'Saiba mais')), items: [
  SubmenuTitleProps(title: MenuTextProps(textProps: TextProps(Text: 'PRINCIPAIS RECURSOS')), items: [MenuTitleProps(title: TextProps(Text: 'Biblioteca de componentes'), description: TextProps(Text: 'Construa interfaces consistentes com componentes reutilizáveis.'), href: '/main/project'), MenuTitleProps(title: TextProps(Text: 'Editor visual'), description: TextProps(Text: 'Explore e ajuste componentes num espaço interactivo.'), href: '/main/project')]),
  SubmenuTitleProps(title: MenuTextProps(textProps: TextProps(Text: 'DESCOBRIR')), items: [MenuTitleProps(title: TextProps(Text: 'Documentação'), description: TextProps(Text: 'Consulte padrões, propriedades e exemplos.'), href: '/main/project'), MenuTitleProps(title: TextProps(Text: 'Actualizações'), description: TextProps(Text: 'Veja as novidades mais recentes.'), href: '/main/project')]),
]);

class SubmenuGridTitleMenu extends StatelessWidget {
  const SubmenuGridTitleMenu({super.key});
  @override
  Widget build(BuildContext context) => Menu(MenuProps(logo: const ImageProps(src: '/img/programmer-01.jpg', alt: 'Negodesign'), menus: const [MenuTextProps(href: '/', icon: IconProps(name: 'Home'), textProps: TextProps(Text: 'Início')), platformMenu, MenuTextProps(href: '/main/project', textProps: TextProps(Text: 'Equipa'))], buttonsProps: const [ButtonProps(text: 'Entrar', variant: 'outline'), ButtonProps(text: 'Criar conta', variant: 'solid')], defaultOpenSubmenu: 1));
}
