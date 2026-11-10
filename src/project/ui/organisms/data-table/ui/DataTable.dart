import 'package:flutter/material.dart';
import '../../../../data/theme/theme_data.dart';
import '../data/types.dart';
import '../data/viewmodel.dart';
import '../shared/data-table-filter/data/types.dart';
import '../shared/data-table-filter/ui/DataTableFilter.dart';
import 'flutter/data_table_cell.dart';

class DataTableWidget extends StatefulWidget {
  const DataTableWidget({super.key, this.variant = '01', this.title, this.columns, this.rows});
  final String variant;
  final String? title;
  final List<TextSegment>? columns;
  final List<TextSegment>? rows;

  @override
  State<DataTableWidget> createState() => _DataTableWidgetState();
}

class _DataTableWidgetState extends State<DataTableWidget> {
  late final DataTableViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = DataTableViewModel(variant: widget.variant, suppliedColumns: widget.columns, suppliedRows: widget.rows);
  }

  @override
  void didUpdateWidget(covariant DataTableWidget oldWidget) {
    super.didUpdateWidget(oldWidget);
    _vm.didUpdate(variant: widget.variant, columns: widget.columns, rows: widget.rows);
  }

  @override
  void dispose() {
    _vm.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: _vm,
      builder: (context, _) {
        final cols = _vm.columns;
        final rows = _vm.visibleRows;
        return Container(
          decoration: NegoThemeData.cardDecoration(),
          clipBehavior: Clip.antiAlias,
          child: Column(crossAxisAlignment: CrossAxisAlignment.stretch, children: [
            if (widget.title != null)
              Padding(
                padding: const EdgeInsets.fromLTRB(16, 16, 16, 0),
                child: Text(widget.title!, style: const TextStyle(fontWeight: FontWeight.w600, color: NegoThemeData.textPrimary)),
              ),
            DataTableFilterWidget(
              props: DataTableFilterProps(
                columns: cols, filterOptions: _vm.filterOptions, searchQuery: _vm.search,
                categoryFilter: _vm.category, hiddenColumns: _vm.hiddenCols,
                onSearchChange: _vm.setSearch, onCategoryChange: _vm.setCategory,
                onToggleColumn: _vm.toggleColumn, onReset: _vm.reset,
              ),
            ),
            SingleChildScrollView(
              scrollDirection: Axis.horizontal,
              child: DataTable(
                headingRowColor: const WidgetStatePropertyAll(NegoThemeData.blenderBase),
                columns: [
                  for (var i = 0; i < cols.length; i++)
                    if (!_vm.hiddenCols.contains(i))
                      DataColumn(label: Text(cols[i].text, style: const TextStyle(fontSize: 11, color: NegoThemeData.textMuted)), onSort: (_, __) => _vm.toggleSort()),
                ],
                rows: [
                  for (var ri = 0; ri < rows.length; ri++)
                    DataRow(
                      selected: _vm.selected.contains(ri),
                      onSelectChanged: (v) => _vm.toggleRowSelection(ri, v),
                      cells: [
                        for (var ci = 0; ci < rows[ri].length; ci++)
                          if (!_vm.hiddenCols.contains(ci))
                            DataCell(DataTableCellText(text: rows[ri][ci].text, className: rows[ri][ci].className)),
                      ],
                    ),
                ],
              ),
            ),
          ]),
        );
      },
    );
  }
}
