import 'package:flutter/foundation.dart';
import 'types.dart';

class UnderlineNavViewModel extends ChangeNotifier {
  UnderlineNavProps _props;
  late String _active;

  UnderlineNavViewModel(this._props) {
    _active = _props.value;
  }

  UnderlineNavProps get props => _props;
  String get active => _active;

  void didUpdate(UnderlineNavProps nextProps) {
    _props = nextProps;
    _active = nextProps.value;
    notifyListeners();
  }

  void select(String value) {
    _active = value;
    _props.onValueChange?.call(value);
    notifyListeners();
  }
}
