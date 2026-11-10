import 'package:flutter/material.dart';
import 'bootswatch_palette.dart';
import 'bootswatch_part1.dart';
import 'bootswatch_part2.dart';
import 'bootswatch_part3.dart';
import 'bootswatch_part4.dart';
import 'bootswatch_part5.dart';
import 'bootswatch_part6.dart';
import 'bootswatch_part7.dart';

export 'bootswatch_palette.dart';

class NegoThemeData {
  const NegoThemeData._();

  static const Color blenderBase = Color(0xFF12121A);
  static const Color blenderSurface = Color(0xFF16161E);
  static const Color blenderCard = Color(0xFF1E1E2E);
  static const Color blenderBorder = Color(0xFF2E2E3E);
  static const Color blenderHover = Color(0xFF262636);
  static const Color primary = Color(0xFF3B82F6);
  static const Color textPrimary = Color(0xFFF4F4F5);
  static const Color textSecondary = Color(0xFFD4D4D8);
  static const Color textMuted = Color(0xFF71717A);
  static const Color danger = Color(0xFFDC2626);
  static const Color success = Color(0xFF059669);
  static const Color warning = Color(0xFFD97706);

  static const double radiusSm = 4;
  static const double radiusMd = 6;
  static const double radiusLg = 8;
  static const double radiusXl = 12;

  static const Map<String, NegoBootswatchPalette> bootswatchThemes = {
    ...kBootswatchPart1,
    ...kBootswatchPart2,
    ...kBootswatchPart3,
    ...kBootswatchPart4,
    ...kBootswatchPart5,
    ...kBootswatchPart6,
    ...kBootswatchPart7,
  };

  static NegoBootswatchPalette? getBootswatchPalette(String id) => bootswatchThemes[id];

  static BoxDecoration cardDecoration({double radius = radiusXl}) => BoxDecoration(
        color: blenderCard,
        border: Border.all(color: blenderBorder),
        borderRadius: BorderRadius.circular(radius),
      );

  static BoxDecoration surfaceDecoration({
    double radius = radiusMd,
    Color? borderColor,
  }) =>
      BoxDecoration(
        color: blenderSurface,
        border: Border.all(color: borderColor ?? blenderBorder),
        borderRadius: BorderRadius.circular(radius),
      );

  static ThemeData toMaterialTheme([String? bootswatchId]) {
    if (bootswatchId != null && bootswatchThemes.containsKey(bootswatchId)) {
      return bootswatchThemes[bootswatchId]!.toMaterialTheme();
    }
    return ThemeData.dark(useMaterial3: true).copyWith(
      scaffoldBackgroundColor: blenderBase,
      cardColor: blenderCard,
      dividerColor: blenderBorder,
      colorScheme: const ColorScheme.dark(
        primary: primary,
        surface: blenderCard,
        error: danger,
      ),
    );
  }
}
