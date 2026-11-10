import 'package:flutter/material.dart';
import '../../../../../../data/theme/theme_data.dart';
import '../../../../data/types.dart';

class FilterToolButton extends StatelessWidget {
  const FilterToolButton({
    super.key,
    required this.icon,
    required this.label,
    this.active = false,
    required this.onTap,
  });
  final IconData icon;
  final String label;
  final bool active;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    final borderColor = active ? NegoThemeData.primary.withOpacity(0.6) : NegoThemeData.blenderBorder;
    final fg = active ? NegoThemeData.primary : NegoThemeData.textSecondary;
    return GestureDetector(
      onTap: onTap,
      child: Container(
        height: 36,
        padding: const EdgeInsets.symmetric(horizontal: 12),
        decoration: NegoThemeData.surfaceDecoration(borderColor: borderColor),
        child: Row(mainAxisSize: MainAxisSize.min, children: [
          Icon(icon, size: 15, color: fg),
          const SizedBox(width: 6),
          Text(label, style: TextStyle(fontSize: 13, fontWeight: FontWeight.w500, color: fg)),
        ]),
      ),
    );
  }
}

class FilterColumnsPanel extends StatelessWidget {
  const FilterColumnsPanel({
    super.key,
    required this.columns,
    required this.hiddenColumns,
    this.onToggleColumn,
  });
  final List<TextSegment> columns;
  final Set<int> hiddenColumns;
  final ValueChanged<int>? onToggleColumn;

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: const BoxDecoration(
        color: NegoThemeData.blenderCard,
        border: Border(bottom: BorderSide(color: NegoThemeData.blenderBorder)),
      ),
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: columns.asMap().entries.map((e) {
          final ci = e.key;
          final hidden = hiddenColumns.contains(ci);
          final isFirst = ci == 0;
          return InkWell(
            onTap: isFirst ? null : () => onToggleColumn?.call(ci),
            child: Padding(
              padding: const EdgeInsets.symmetric(vertical: 6),
              child: Row(children: [
                Icon(
                  hidden ? Icons.check_box_outline_blank : Icons.check_box,
                  size: 18,
                  color: isFirst ? NegoThemeData.textMuted : (hidden ? NegoThemeData.textMuted : NegoThemeData.primary),
                ),
                const SizedBox(width: 10),
                Expanded(
                  child: Text(
                    e.value.text,
                    style: TextStyle(
                      fontSize: 13,
                      color: isFirst ? NegoThemeData.textMuted : (hidden ? NegoThemeData.textMuted : NegoThemeData.textPrimary),
                    ),
                  ),
                ),
                if (!isFirst) const Icon(Icons.drag_indicator, size: 16, color: NegoThemeData.textMuted),
              ]),
            ),
          );
        }).toList(),
      ),
    );
  }
}
