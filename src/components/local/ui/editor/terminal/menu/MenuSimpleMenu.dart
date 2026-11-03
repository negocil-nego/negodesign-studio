import 'package:flutter/material.dart';
import '../../../../../../project/ui/organisms/menu/simple/ui/Menu.dart';
import '../../../../../../project/ui/organisms/menu/simple/data/types.dart';
import '../../../../../../project/ui/organisms/menu/shared/menu/menu-text/data/types.dart';
import '../../../../../../project/ui/atoms/text/data/types.dart';
import '../../../../../../project/ui/atoms/image/data/types.dart';
import '../../../../../../project/ui/atoms/button/variant/default/type/types.dart';
import '../../../../../../project/ui/atoms/icon/data/types.dart';

class MenuSimpleExample extends StatelessWidget {
  const MenuSimpleExample({super.key});

  @override
  Widget build(BuildContext context) => Menu(MenuProps(
    logo: const ImageProps(src: '/img/programmer-01.jpg', alt: 'Negodesign'),
    menus: const [
      MenuTextProps(href: '/', icon: IconProps(name: 'Home'), textProps: TextProps(Text: 'Início')),
      MenuTextProps(href: '/projectos', textProps: TextProps(Text: 'Projectos')),
      MenuTextProps(href: '/equipa', textProps: TextProps(Text: 'Equipa')),
    ],
    buttonsProps: const [
      ButtonProps(text: 'Entrar', variant: 'outline'),
      ButtonProps(text: 'Criar conta', variant: 'solid'),
    ],
  ));
}
