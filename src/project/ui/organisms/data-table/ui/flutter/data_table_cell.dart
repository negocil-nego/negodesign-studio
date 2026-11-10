import 'package:flutter/material.dart';
import '../../../../../data/theme/theme_data.dart';

class DataTableCellText extends StatelessWidget {
  const DataTableCellText({super.key, required this.text, this.className});
  final String text;
  final String? className;

  @override
  Widget build(BuildContext context) {
    Color? color;
    var hasBadge = false;
    if (className != null) {
      if (className!.contains('text-red') || className!.contains('text-rose')) color = NegoThemeData.danger;
      if (className!.contains('text-orange') || className!.contains('text-amber')) color = NegoThemeData.warning;
      if (className!.contains('text-emerald')) color = NegoThemeData.success;
      hasBadge = className!.contains('rounded') || className!.contains('bg-');
    }
    if (hasBadge) {
      final badgeColor = color ?? NegoThemeData.textMuted;
      return Container(
        margin: const EdgeInsets.only(right: 2),
        padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
        decoration: BoxDecoration(
          color: badgeColor.withOpacity(0.15),
          borderRadius: BorderRadius.circular(4),
          border: Border.all(color: badgeColor.withOpacity(0.4)),
        ),
        child: Text(text, style: TextStyle(fontSize: 11, color: color ?? NegoThemeData.textSecondary)),
      );
    }
    return Text(text, style: TextStyle(fontSize: 12, color: color ?? NegoThemeData.textSecondary));
  }
}
