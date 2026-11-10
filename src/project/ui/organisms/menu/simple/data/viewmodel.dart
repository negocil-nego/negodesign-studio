import 'package:flutter/material.dart';
import '../../shared/submenu/grid/title/data/types.dart';
import '../../shared/submenu/grid/sidebar/data/types.dart';
import 'types.dart';

class MenuViewModel extends ChangeNotifier {
  MenuProps props;
  int? openIndex;

  MenuViewModel(this.props) {
    openIndex = props.defaultOpenSubmenu;
  }

  void didUpdate(MenuProps newProps) {
    props = newProps;
  }

  Object? get activeMenu =>
      openIndex != null && openIndex! < props.menus.length ? props.menus[openIndex!] : null;

  bool get hasPanel => activeMenu is SubmenuSidebarGridProps || activeMenu is SubmenuGridTitleProps;

  String labelFor(int index) {
    final m = props.menus[index];
    if (m is SubmenuSidebarGridProps) return m.title?.title?.Text ?? 'Explore';
    if (m is SubmenuGridTitleProps) return m.label ?? 'Explore';
    return 'Menu ${index + 1}';
  }

  void toggleSubmenu(int index) {
    openIndex = openIndex == index ? null : index;
    notifyListeners();
  }
}

