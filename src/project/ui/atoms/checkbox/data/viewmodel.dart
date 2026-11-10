import 'package:flutter/material.dart';
import 'types.dart';

class CheckboxViewModel extends ChangeNotifier {
  CheckboxProps props;
  late bool checked;

  CheckboxViewModel(this.props) {
    checked = props.checked;
  }

  void didUpdate(CheckboxProps newProps) {
    if (props.checked != newProps.checked) {
      checked = newProps.checked;
    }
    props = newProps;
  }

  void toggle(bool? value) {
    checked = value ?? false;
    props.onchange?.call(checked);
    notifyListeners();
  }
}

