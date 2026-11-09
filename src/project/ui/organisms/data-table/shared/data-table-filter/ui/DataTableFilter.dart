import 'package:flutter/material.dart';
import '../data/types.dart';

const _cardBg  = Color(0xFF1E1E2E);
const _surface = Color(0xFF16161E);
const _border  = Color(0xFF2E2E3E);
const _zinc100 = Color(0xFFF4F4F5);
const _zinc300 = Color(0xFFD4D4D8);
const _zinc400 = Color(0xFFA1A1AA);
const _zinc500 = Color(0xFF71717A);
const _blue400 = Color(0xFF60A5FA);
const _blue500 = Color(0xFF3B82F6);

class DataTableFilterWidget extends StatefulWidget {
  const DataTableFilterWidget({super.key, required this.props});
  final DataTableFilterProps props;

  @override
  State<DataTableFilterWidget> createState() => _DataTableFilterWidgetState();
}

class _DataTableFilterWidgetState extends State<DataTableFilterWidget> {
  bool _showColPanel = false;
  late final TextEditingController _searchController;

  @override
  void initState() {
    super.initState();
    _searchController = TextEditingController(text: widget.props.searchQuery);
  }

  @override
  void didUpdateWidget(covariant DataTableFilterWidget oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.props.searchQuery != _searchController.text) {
      _searchController.text = widget.props.searchQuery;
    }
  }

  @override
  void dispose() {
    _searchController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final p = widget.props;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Container(
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
          decoration: const BoxDecoration(border: Border(bottom: BorderSide(color: _border))),
          child: Wrap(
            spacing: 8,
            runSpacing: 8,
            alignment: WrapAlignment.spaceBetween,
            crossAxisAlignment: WrapCrossAlignment.center,
            children: [
              Container(
                width: 240,
                height: 36,
                decoration: BoxDecoration(
                  color: _surface,
                  border: Border.all(color: _border),
                  borderRadius: BorderRadius.circular(6),
                ),
                padding: const EdgeInsets.symmetric(horizontal: 12),
                child: Row(children: [
                  const Icon(Icons.search, size: 16, color: _zinc500),
                  const SizedBox(width: 8),
                  Expanded(
                    child: TextField(
                      controller: _searchController,
                      style: const TextStyle(fontSize: 13, color: _zinc300),
                      decoration: const InputDecoration(
                        hintText: 'Search',
                        hintStyle: TextStyle(color: _zinc500),
                        border: InputBorder.none,
                        isDense: true,
                        contentPadding: EdgeInsets.zero,
                      ),
                      onChanged: (v) => p.onSearchChange?.call(v),
                    ),
                  ),
                ]),
              ),
              Wrap(
                spacing: 8,
                crossAxisAlignment: WrapCrossAlignment.center,
                children: [
                  Container(
                    height: 36,
                    padding: const EdgeInsets.symmetric(horizontal: 12),
                    decoration: BoxDecoration(
                      color: _surface,
                      border: Border.all(color: _border),
                      borderRadius: BorderRadius.circular(6),
                    ),
                    child: DropdownButtonHideUnderline(
                      child: DropdownButton<String>(
                        value: p.categoryFilter,
                        dropdownColor: _cardBg,
                        icon: const Icon(Icons.keyboard_arrow_down, size: 16, color: _zinc500),
                        style: const TextStyle(fontSize: 13, color: _zinc300),
                        items: [
                          const DropdownMenuItem(value: 'all', child: Text('All Campaigns')),
                          ...p.filterOptions.map((opt) => DropdownMenuItem(value: opt, child: Text(opt))),
                        ],
                        onChanged: (val) => p.onCategoryChange?.call(val ?? 'all'),
                      ),
                    ),
                  ),
                  _toolBtn(icon: Icons.filter_list, label: 'Filter', onTap: () {}),
                  _toolBtn(
                    icon: Icons.view_column_outlined,
                    label: 'Columns',
                    active: _showColPanel,
                    onTap: () => setState(() => _showColPanel = !_showColPanel),
                  ),
                  _iconBtn(
                    icon: Icons.more_vert,
                    onTap: () => p.onReset?.call(),
                  ),
                ],
              ),
            ],
          ),
        ),
        if (_showColPanel)
          Container(
            decoration: const BoxDecoration(
              color: _cardBg,
              border: Border(bottom: BorderSide(color: _border)),
            ),
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: p.columns.asMap().entries.map((e) {
                final ci = e.key;
                final col = e.value;
                final hidden = p.hiddenColumns.contains(ci);
                final isFirst = ci == 0;
                return InkWell(
                  onTap: isFirst ? null : () => p.onToggleColumn?.call(ci),
                  child: Padding(
                    padding: const EdgeInsets.symmetric(vertical: 6),
                    child: Row(children: [
                      Icon(
                        hidden ? Icons.check_box_outline_blank : Icons.check_box,
                        size: 18,
                        color: isFirst ? _zinc500 : (hidden ? _zinc500 : _blue500),
                      ),
                      const SizedBox(width: 10),
                      Expanded(
                        child: Text(
                          col.text,
                          style: TextStyle(
                            fontSize: 13,
                            color: isFirst ? _zinc400 : (hidden ? _zinc500 : _zinc100),
                          ),
                        ),
                      ),
                      if (!isFirst)
                        const Icon(Icons.drag_indicator, size: 16, color: _zinc500),
                    ]),
                  ),
                );
              }).toList(),
            ),
          ),
      ],
    );
  }

  Widget _toolBtn({
    required IconData icon,
    required String label,
    bool active = false,
    required VoidCallback onTap,
  }) {
    return GestureDetector(
      onTap: onTap,
      child: Container(
        height: 36,
        padding: const EdgeInsets.symmetric(horizontal: 12),
        decoration: BoxDecoration(
          color: _surface,
          border: Border.all(color: active ? _blue500.withOpacity(0.6) : _border),
          borderRadius: BorderRadius.circular(6),
        ),
        child: Row(mainAxisSize: MainAxisSize.min, children: [
          Icon(icon, size: 15, color: active ? _blue400 : _zinc300),
          const SizedBox(width: 6),
          Text(label, style: TextStyle(fontSize: 13, fontWeight: FontWeight.w500, color: active ? _blue400 : _zinc300)),
        ]),
      ),
    );
  }

  Widget _iconBtn({required IconData icon, required VoidCallback onTap}) {
    return GestureDetector(
      onTap: onTap,
      child: Container(
        width: 36,
        height: 36,
        decoration: BoxDecoration(
          color: _surface,
          border: Border.all(color: _border),
          borderRadius: BorderRadius.circular(6),
        ),
        child: Icon(icon, size: 16, color: _zinc400),
      ),
    );
  }
}
