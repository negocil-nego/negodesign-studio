import 'package:flutter/material.dart';
import '../../../../../data/theme/theme_data.dart';
import '../data/types.dart';
import '../data/viewmodel.dart';
import 'flutter/filter_controls.dart';

class DataTableFilterWidget extends StatefulWidget {
  const DataTableFilterWidget({super.key, required this.props});
  final DataTableFilterProps props;

  @override
  State<DataTableFilterWidget> createState() => _DataTableFilterWidgetState();
}

class _DataTableFilterWidgetState extends State<DataTableFilterWidget> {
  late final DataTableFilterViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = DataTableFilterViewModel(widget.props);
  }

  @override
  void didUpdateWidget(covariant DataTableFilterWidget oldWidget) {
    super.didUpdateWidget(oldWidget);
    _vm.didUpdate(widget.props);
  }

  @override
  void dispose() {
    _vm.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final p = widget.props;
    return ListenableBuilder(
      listenable: _vm,
      builder: (context, _) => Column(crossAxisAlignment: CrossAxisAlignment.stretch, children: [
        Container(
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
          decoration: const BoxDecoration(border: Border(bottom: BorderSide(color: NegoThemeData.blenderBorder))),
          child: Wrap(
            spacing: 8,
            runSpacing: 8,
            alignment: WrapAlignment.spaceBetween,
            crossAxisAlignment: WrapCrossAlignment.center,
            children: [
              Container(
                width: 240,
                height: 36,
                decoration: NegoThemeData.surfaceDecoration(),
                padding: const EdgeInsets.symmetric(horizontal: 12),
                child: Row(children: [
                  const Icon(Icons.search, size: 16, color: NegoThemeData.textMuted),
                  const SizedBox(width: 8),
                  Expanded(
                    child: TextField(
                      controller: _vm.searchCtrl,
                      style: const TextStyle(fontSize: 13, color: NegoThemeData.textSecondary),
                      decoration: const InputDecoration(hintText: 'Search', hintStyle: TextStyle(color: NegoThemeData.textMuted), border: InputBorder.none, isDense: true),
                      onChanged: (v) => p.onSearchChange?.call(v),
                    ),
                  ),
                ]),
              ),
              Wrap(spacing: 8, crossAxisAlignment: WrapCrossAlignment.center, children: [
                Container(
                  height: 36,
                  padding: const EdgeInsets.symmetric(horizontal: 12),
                  decoration: NegoThemeData.surfaceDecoration(),
                  child: DropdownButtonHideUnderline(
                    child: DropdownButton<String>(
                      value: p.categoryFilter,
                      dropdownColor: NegoThemeData.blenderCard,
                      icon: const Icon(Icons.keyboard_arrow_down, size: 16, color: NegoThemeData.textMuted),
                      style: const TextStyle(fontSize: 13, color: NegoThemeData.textSecondary),
                      items: [const DropdownMenuItem(value: 'all', child: Text('All Campaigns')), ...p.filterOptions.map((o) => DropdownMenuItem(value: o, child: Text(o)))],
                      onChanged: (val) => p.onCategoryChange?.call(val ?? 'all'),
                    ),
                  ),
                ),
                FilterToolButton(icon: Icons.filter_list, label: 'Filter', onTap: () {}),
                FilterToolButton(icon: Icons.view_column_outlined, label: 'Columns', active: _vm.showColPanel, onTap: _vm.toggleColPanel),
                FilterToolButton(icon: Icons.more_vert, label: '', onTap: () => p.onReset?.call()),
              ]),
            ],
          ),
        ),
        if (_vm.showColPanel) FilterColumnsPanel(columns: p.columns, hiddenColumns: p.hiddenColumns, onToggleColumn: p.onToggleColumn),
      ]),
    );
  }
}
