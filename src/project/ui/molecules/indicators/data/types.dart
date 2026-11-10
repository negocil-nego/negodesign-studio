import 'package:flutter/foundation.dart';

class AlertItem {
  final String id;
  final String variant;
  final String? heading;
  final String message;
  final bool dismissible;

  const AlertItem({
    required this.id,
    required this.variant,
    this.heading,
    required this.message,
    this.dismissible = true,
  });
}

class BadgeItem {
  final String label;
  final String variant;
  final bool pill;

  const BadgeItem({
    required this.label,
    required this.variant,
    this.pill = false,
  });
}

class IndicatorsProps {
  final List<AlertItem> alerts;
  final List<BadgeItem> badges;
  final ValueChanged<String>? onDismissAlert;

  const IndicatorsProps({
    this.alerts = const [],
    this.badges = const [],
    this.onDismissAlert,
  });
}
