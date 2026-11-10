import 'package:flutter/foundation.dart';
import 'types.dart';

class InputRangeViewModel extends ChangeNotifier {
  InputRangeProps _props;
  late double _value;

  InputRangeViewModel(this._props) {
    _value = _props.value.clamp(_props.min, _props.max);
  }

  InputRangeProps get props => _props;
  double get value => _value;
  int? get divisions => _props.step > 0 ? ((_props.max - _props.min) / _props.step).round() : null;

  void didUpdate(InputRangeProps nextProps) {
    if (nextProps.value != _props.value) {
      _value = nextProps.value.clamp(nextProps.min, nextProps.max);
    }
    _props = nextProps;
    notifyListeners();
  }

  void updateValue(double next) {
    if (_props.disabled) return;
    _value = next.clamp(_props.min, _props.max);
    _props.onValueChange?.call(_value);
    notifyListeners();
  }
}
