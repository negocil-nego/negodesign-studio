import 'package:flutter/foundation.dart';
import '../../../data/types.dart';

class DataTableFilterProps {
  const DataTableFilterProps({
    required this.columns,
    this.filterOptions = const [],
    this.searchQuery = '',
    this.categoryFilter = 'all',
    this.hiddenColumns = const {},
    this.onSearchChange,
    this.onCategoryChange,
    this.onToggleColumn,
    this.onReset,
  });

  final List<TextSegment> columns;
  final List<String> filterOptions;
  final String searchQuery;
  final String categoryFilter;
  final Set<int> hiddenColumns;
  final ValueChanged<String>? onSearchChange;
  final ValueChanged<String>? onCategoryChange;
  final ValueChanged<int>? onToggleColumn;
  final VoidCallback? onReset;
}
