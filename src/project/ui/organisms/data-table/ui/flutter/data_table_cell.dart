import 'package:flutter/material.dart';
import '../../../../../data/theme/dart/theme_data.dart';
import '../../data/types.dart';

class DataTableCellText extends StatelessWidget {
  const DataTableCellText({
    super.key,
    required this.cell,
    this.column,
    this.defaultActions,
    this.onAction,
  });

  final TextSegment cell;
  final TextSegment? column;
  final List<String>? defaultActions;
  final ValueChanged<String>? onAction;

  @override
  Widget build(BuildContext context) {
    final kind = cell.type ?? cell.component ?? column?.type ?? column?.component;
    if (kind == 'dropdown' || kind == 'actions' || (cell.actions != null && cell.actions!.isNotEmpty)) {
      final items = cell.actions ?? column?.actions ?? defaultActions ?? const ['Edit', 'Details', 'Delete'];
      return PopupMenuButton<String>(
        icon: const Icon(Icons.more_horiz, size: 18, color: NegoThemeData.textSecondary),
        color: NegoThemeData.blenderCard,
        onSelected: (v) => onAction?.call(v),
        itemBuilder: (context) => [
          for (final item in items)
            PopupMenuItem<String>(
              value: item.toLowerCase(),
              child: Text(
                item,
                style: TextStyle(
                  fontSize: 12,
                  color: item.toLowerCase().contains('delete') ? NegoThemeData.danger : NegoThemeData.textPrimary,
                ),
              ),
            ),
        ],
      );
    }

    Color? color;
    var hasBadge = kind == 'badge' || cell.badgeVariant != null;
    if (cell.badgeVariant != null) {
      if (cell.badgeVariant == 'danger') color = NegoThemeData.danger;
      if (cell.badgeVariant == 'warning') color = NegoThemeData.warning;
      if (cell.badgeVariant == 'success') color = NegoThemeData.success;
      if (cell.badgeVariant == 'primary' || cell.badgeVariant == 'info') color = Colors.blueAccent;
    }
    if (cell.className != null) {
      if (cell.className!.contains('text-red') || cell.className!.contains('text-rose')) color = NegoThemeData.danger;
      if (cell.className!.contains('text-orange') || cell.className!.contains('text-amber')) color = NegoThemeData.warning;
      if (cell.className!.contains('text-emerald')) color = NegoThemeData.success;
      hasBadge = hasBadge || cell.className!.contains('rounded') || cell.className!.contains('bg-');
    }

    if (hasBadge) {
      final badgeColor = color ?? NegoThemeData.textMuted;
      return Container(
        margin: const EdgeInsets.only(right: 2),
        padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
        decoration: BoxDecoration(
          color: badgeColor.withOpacity(0.15),
          borderRadius: BorderRadius.circular(999),
          border: Border.all(color: badgeColor.withOpacity(0.4)),
        ),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            if (cell.dot) ...[
              Container(width: 6, height: 6, decoration: BoxDecoration(color: badgeColor, shape: BoxShape.circle)),
              const SizedBox(width: 5),
            ],
            Text(cell.text, style: TextStyle(fontSize: 11, fontWeight: FontWeight.w500, color: color ?? NegoThemeData.textSecondary)),
          ],
        ),
      );
    }
    return Text(cell.text, style: TextStyle(fontSize: 12, color: color ?? NegoThemeData.textSecondary));
  }
}
