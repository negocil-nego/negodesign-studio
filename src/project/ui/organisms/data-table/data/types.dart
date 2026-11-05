class DataTableColumn {
  const DataTableColumn({required this.label, required this.key, this.sortable = true});
  final String label;
  final String key;
  final bool sortable;
}

class DataTableRowData {
  const DataTableRowData({required this.id, required this.cells, this.avatar, this.badge});
  final String id;
  final List<String> cells;
  final String? avatar;
  final String? badge;
}
