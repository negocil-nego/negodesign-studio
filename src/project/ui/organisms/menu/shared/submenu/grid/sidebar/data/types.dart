import '../../../../menu/menu-title/data/types.dart';
import '../../../simple/title/data/types.dart';
class SubmenuSidebarGridProps { final String type; final MenuTitleProps? title; final List<SubmenuTitleProps> items; final int defaultSelected; const SubmenuSidebarGridProps({this.type = 'sidebar-grid', this.title, this.items = const [], this.defaultSelected = 0}); }
