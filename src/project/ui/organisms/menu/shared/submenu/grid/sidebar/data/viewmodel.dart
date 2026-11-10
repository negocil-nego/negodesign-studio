import 'package:flutter/material.dart';
import 'types.dart';

class SubmenuSidebarGridViewModel extends ChangeNotifier {
  SubmenuSidebarGridProps props;
  late int selected;

  SubmenuSidebarGridViewModel(this.props) {
    selected = props.defaultSelected.clamp(0, (props.items.length - 1).clamp(0, 999));
  }

  void didUpdate(SubmenuSidebarGridProps newProps) {
    props = newProps;
    selected = selected.clamp(0, (props.items.length - 1).clamp(0, 999));
  }

  void selectIndex(int index) {
    selected = index;
    notifyListeners();
  }
}

