import 'package:flutter/material.dart';
import '../../../../data/theme/dart/theme_data.dart';
import 'types.dart';

class IndicatorsViewModel extends ChangeNotifier {
  IndicatorsProps _props;
  final Set<String> _dismissed = {};

  IndicatorsViewModel(this._props);

  List<AlertItem> get visibleAlerts =>
      _props.alerts.where((a) => !_dismissed.contains(a.id)).toList();

  List<BadgeItem> get badges => _props.badges;

  void didUpdate(IndicatorsProps nextProps) {
    _props = nextProps;
    notifyListeners();
  }

  void dismiss(String id) {
    _dismissed.add(id);
    _props.onDismissAlert?.call(id);
    notifyListeners();
  }

  Color variantColor(String variant, Color primary) {
    switch (variant) {
      case 'success':
        return NegoThemeData.success;
      case 'danger':
        return NegoThemeData.danger;
      case 'warning':
        return NegoThemeData.warning;
      case 'secondary':
        return NegoThemeData.blenderHover;
      default:
        return primary;
    }
  }
}
