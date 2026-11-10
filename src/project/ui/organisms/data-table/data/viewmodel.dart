import 'package:flutter/material.dart';
import 'types.dart';
import '../ui/flutter/data_table_examples.dart';

class DataTableViewModel extends ChangeNotifier {
  String variant;
  List<TextSegment>? suppliedColumns;
  List<TextSegment>? suppliedRows;

  bool sortAsc = true;
  String search = '';
  String category = 'all';
  final Set<int> selected = {};
  final Set<int> hiddenCols = {};

  DataTableViewModel({this.variant = '01', this.suppliedColumns, this.suppliedRows});

  void didUpdate({required String variant, List<TextSegment>? columns, List<TextSegment>? rows}) {
    this.variant = variant;
    suppliedColumns = columns;
    suppliedRows = rows;
  }

  List<TextSegment> get columns => suppliedColumns ?? DataTableExamplesFlutter.columns(variant);

  List<List<TextSegment>> get _allRows {
    final cols = columns;
    final flat = suppliedRows ?? DataTableExamplesFlutter.rows(variant);
    final n = cols.isEmpty ? 1 : cols.length;
    return [for (var i = 0; i < flat.length; i += n) flat.sublist(i, (i + n).clamp(0, flat.length))];
  }

  List<String> get filterOptions {
    final colIdx = columns.length > 1 ? 1 : 0;
    return {
      for (final r in _allRows)
        if (colIdx < r.length && r[colIdx].text.isNotEmpty) r[colIdx].text,
    }.toList();
  }

  List<List<TextSegment>> get visibleRows {
    final colIdx = columns.length > 1 ? 1 : 0;
    var rows = sortAsc ? [..._allRows] : _allRows.reversed.toList();
    if (category != 'all') {
      rows = rows.where((r) => colIdx < r.length && r[colIdx].text == category).toList();
    }
    if (search.isNotEmpty) {
      final q = search.toLowerCase();
      rows = rows.where((r) => r.any((c) => c.text.toLowerCase().contains(q))).toList();
    }
    return rows;
  }

  void setSearch(String value) { search = value; notifyListeners(); }
  void setCategory(String value) { category = value; notifyListeners(); }
  void toggleSort() { sortAsc = !sortAsc; notifyListeners(); }

  void toggleColumn(int ci) {
    if (hiddenCols.contains(ci)) {
      hiddenCols.remove(ci);
    } else if (hiddenCols.length < columns.length - 1) {
      hiddenCols.add(ci);
    }
    notifyListeners();
  }

  void toggleRowSelection(int ri, bool? isSelected) {
    isSelected == true ? selected.add(ri) : selected.remove(ri);
    notifyListeners();
  }

  void reset() {
    search = '';
    category = 'all';
    hiddenCols.clear();
    notifyListeners();
  }
}
