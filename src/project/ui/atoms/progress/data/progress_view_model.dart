import 'package:flutter/material.dart';
import '../../../../data/theme/dart/theme_data.dart';
import 'types.dart';

class ProgressViewModel extends ChangeNotifier {
  ProgressProps _props;

  ProgressViewModel(this._props);

  ProgressProps get props => _props;

  List<ProgressBarSegment> get resolvedBars => _props.bars.isNotEmpty
      ? _props.bars
      : [
          ProgressBarSegment(
            value: _props.value,
            variant: _props.variant,
            striped: _props.striped,
            animated: _props.animated,
          ),
        ];

  void didUpdate(ProgressProps nextProps) {
    _props = nextProps;
    notifyListeners();
  }

  Color resolveColor(String variant, Color primary) {
    switch (variant) {
      case 'success':
        return NegoThemeData.success;
      case 'warning':
        return NegoThemeData.warning;
      case 'danger':
        return NegoThemeData.danger;
      default:
        return primary;
    }
  }
}
