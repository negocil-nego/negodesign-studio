import '../../../../atoms/button/variant/default/type/types.dart';
import '../../../../atoms/image/data/types.dart';
import '../../shared/menu/menu-text/data/types.dart';
import '../../shared/submenu/grid/title/data/types.dart';
import '../../shared/submenu/grid/sidebar/data/types.dart';
class MenuProps { final ImageProps? logo; final List<dynamic> menus; final List<ButtonProps> buttonsProps; final int? defaultOpenSubmenu; const MenuProps({this.logo, this.menus = const [], this.buttonsProps = const [], this.defaultOpenSubmenu}); }
