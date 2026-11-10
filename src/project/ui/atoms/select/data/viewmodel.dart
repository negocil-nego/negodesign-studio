import 'package:flutter/material.dart';
import 'types.dart';

class SelectViewModel extends ChangeNotifier {
  final SelectProps props;
  final ValueChanged<SelectOption>? onSelect;
  SelectOption? selected;
  String query = '';

  SelectViewModel(this.props, {this.onSelect}) {
    if (props.value != null && props.options.isNotEmpty) {
      selected = props.options.firstWhere(
        (o) => o.value == props.value || o.label == props.value,
        orElse: () => props.options.first,
      );
    }
  }

  String get displayLabel => selected?.label ?? props.placeholder ?? 'Select...';

  List<SelectOption> get filteredOptions =>
      props.options.where((o) => o.label.toLowerCase().contains(query.toLowerCase())).toList();

  void updateQuery(String value) {
    query = value;
    notifyListeners();
  }

  void applySelection(SelectOption? result) {
    if (result == null) return;
    selected = result;
    onSelect?.call(result);
    notifyListeners();
  }
}
