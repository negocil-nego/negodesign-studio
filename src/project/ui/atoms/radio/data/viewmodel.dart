import 'package:flutter/material.dart';
import 'types.dart';

class RadioViewModel extends ChangeNotifier {
  RadioProps props;
  late String? selected;

  RadioViewModel(this.props) {
    selected = props.value;
  }

  void didUpdate(RadioProps newProps) {
    if (props.value != newProps.value) {
      selected = newProps.value;
    }
    props = newProps;
  }

  void select(String? value) {
    selected = value;
    if (value != null) props.onchange?.call(value);
    notifyListeners();
  }
}

