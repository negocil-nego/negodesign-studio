import 'package:flutter/foundation.dart';
import 'types.dart';

class RadioButtonViewModel extends ChangeNotifier {
  RadioButtonProps _props;
  late String _selected;
  late Set<String> _selectedSet;

  RadioButtonViewModel(this._props) {
    _selected = _props.value;
    _selectedSet = Set<String>.from(_props.values);
  }

  RadioButtonProps get props => _props;

  bool isSelected(String val) =>
      _props.multiple ? _selectedSet.contains(val) : _selected == val;

  void didUpdate(RadioButtonProps nextProps) {
    _props = nextProps;
    _selected = nextProps.value;
    _selectedSet = Set<String>.from(nextProps.values);
    notifyListeners();
  }

  void toggle(String val) {
    if (_props.disabled) return;
    if (_props.multiple) {
      if (_selectedSet.contains(val)) {
        _selectedSet.remove(val);
      } else {
        _selectedSet.add(val);
      }
    } else {
      _selected = val;
      _props.onValueChange?.call(val);
    }
    notifyListeners();
  }
}
