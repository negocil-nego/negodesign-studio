import 'package:flutter/material.dart';

class NegoBootswatchPalette {
  final String id;
  final String name;
  final String description;
  final bool isDark;
  final Color primary;
  final Color secondary;
  final Color success;
  final Color info;
  final Color warning;
  final Color danger;
  final Color bodyBg;
  final Color bodyColor;
  final Color surfaceBg;
  final Color cardBg;
  final Color borderColor;

  const NegoBootswatchPalette({
    required this.id,
    required this.name,
    required this.description,
    required this.isDark,
    required this.primary,
    required this.secondary,
    required this.success,
    required this.info,
    required this.warning,
    required this.danger,
    required this.bodyBg,
    required this.bodyColor,
    required this.surfaceBg,
    required this.cardBg,
    required this.borderColor,
  });

  ThemeData toMaterialTheme() {
    final base = isDark
        ? ThemeData.dark(useMaterial3: true)
        : ThemeData.light(useMaterial3: true);
    final scheme = isDark ? const ColorScheme.dark() : const ColorScheme.light();

    return base.copyWith(
      scaffoldBackgroundColor: bodyBg,
      cardColor: cardBg,
      dividerColor: borderColor,
      colorScheme: scheme.copyWith(
        primary: primary,
        secondary: secondary,
        surface: cardBg,
        error: danger,
        onSurface: bodyColor,
      ),
    );
  }
}
