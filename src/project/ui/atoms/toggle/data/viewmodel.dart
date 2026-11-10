import 'package:flutter/material.dart';
import 'types.dart';

class ToggleViewModel extends ChangeNotifier {
  ToggleProps props;
  late bool checked;

  ToggleViewModel(this.props) {
    checked = props.checked;
  }

  void didUpdate(ToggleProps newProps) {
    if (props.checked != newProps.checked) {
      checked = newProps.checked;
    }
    props = newProps;
  }

  void toggle(bool value) {
    checked = value;
    props.onchange?.call(value);
    notifyListeners();
  }
}

