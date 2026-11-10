import 'package:flutter/material.dart';
import 'types.dart';

class DataTableFilterViewModel extends ChangeNotifier {
  DataTableFilterProps props;
  bool showColPanel = false;
  late final TextEditingController searchCtrl;

  DataTableFilterViewModel(this.props) {
    searchCtrl = TextEditingController(text: props.searchQuery);
  }

  void didUpdate(DataTableFilterProps newProps) {
    props = newProps;
    if (newProps.searchQuery != searchCtrl.text) {
      searchCtrl.text = newProps.searchQuery;
    }
  }

  void toggleColPanel() {
    showColPanel = !showColPanel;
    notifyListeners();
  }

  @override
  void dispose() {
    searchCtrl.dispose();
    super.dispose();
  }
}

