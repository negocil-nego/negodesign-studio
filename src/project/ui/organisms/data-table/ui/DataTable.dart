import 'package:flutter/material.dart';
import '../data/types.dart';
import '../shared/data-table-filter/data/types.dart';
import '../shared/data-table-filter/ui/DataTableFilter.dart';

typedef TextProps = TextSegment;

const _cardBg  = Color(0xFF1E1E2E);
const _surface = Color(0xFF16161E);
const _baseBg  = Color(0xFF12121A);
const _border  = Color(0xFF2E2E3E);
const _hover   = Color(0xFF262636);
const _zinc100 = Color(0xFFF4F4F5);
const _zinc300 = Color(0xFFD4D4D8);
const _zinc500 = Color(0xFF71717A);

class TextWidget extends StatelessWidget {
  const TextWidget({super.key, required this.text, this.className});
  final String text;
  final String? className;

  @override
  Widget build(BuildContext context) {
    Color? color;
    bool hasBadge = false;
    if (className != null) {
      if (className!.contains('text-red'))     color = const Color(0xFFDC2626);
      if (className!.contains('text-orange'))  color = const Color(0xFFEA580C);
      if (className!.contains('text-emerald')) color = const Color(0xFF059669);
      if (className!.contains('text-amber'))   color = const Color(0xFFB45309);
      if (className!.contains('text-rose'))    color = const Color(0xFFE11D48);
      hasBadge = className!.contains('rounded') || className!.contains('bg-');
    }
    if (hasBadge) {
      return Container(
        margin: const EdgeInsets.only(right: 2),
        padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
        decoration: BoxDecoration(
          color: (color ?? _zinc500).withOpacity(0.15),
          borderRadius: BorderRadius.circular(4),
          border: Border.all(color: (color ?? _zinc500).withOpacity(0.4)),
        ),
        child: Text(text, style: TextStyle(fontSize: 11, color: color ?? _zinc300)),
      );
    }
    return Text(text, style: TextStyle(fontSize: 12, color: color ?? _zinc300));
  }
}

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
  bool _sortAsc = true;
  String _search = '';
  String _categoryFilter = 'all';
  final Set<int> _selected = {};
  final Set<int> _hiddenCols = {};

  static List<TextSegment> _fallbackCols(String v) {
    const map = {
      '01': ['Name', 'Role', 'Experience', 'Company', 'Score', 'Date', 'Action'],
      '02': ['Shipment ID', 'Description', 'Location', 'Type', 'Priority', 'Carrier', 'Contact'],
      '03': ['Full name', 'Email', 'Role', 'Status', 'Joined date', '2F Auth', 'Actions'],
      '04': ['Customer', 'Phone', 'Country', 'Balance', 'Total invoice', 'Created On', 'Status', 'Action'],
      '05': ['Name', 'Trading Zones', 'Type', 'ACN', 'Commission % (Buying / Selling)', 'Commission % (Buying / Leasing)'],
    };
    return (map[v] ?? map['01']!).map((l) => TextSegment(text: l)).toList();
  }

  static List<TextSegment> _fallbackRows(String v) {
    TextSegment s(String text, {String? className}) => TextSegment(text: text, className: className);
    final map = <String, List<TextSegment>>{
      '01': [
        s('Maya Thompson'),  s('Whole seller'),     s('3 years'), s('ACE Homes LLC  +2'), s('12,054'), s('12/24/2024'), s(''),
        s('Ethan Carter'),   s('Property manager'), s('3 years'), s('A&H Realty'),        s('12,054'), s('12/24/2024'), s(''),
        s('Sophie Lee'),     s('Investor'),         s('3 years'), s('NestHub LLC  +1'),   s('12,054'), s('12/24/2024'), s(''),
      ],
      '02': [
        s('#SHP-842'), s('Pallet of Electronics'), s('Zone A'), s('Outbound'), s('Urgent', className: 'rounded bg-red-50 text-red-600'), s('FedEx Freight'), s('dispatch@fedex.com'),
        s('#SHP-841'), s('Medical Supplies'),      s('Cold B'), s('Inbound'),  s('Urgent', className: 'rounded bg-red-50 text-red-600'), s('DHL Express'),   s('med@dhl.com'),
      ],
      '03': [
        s('Liam Smith'),    s('smith@example.com'),    s('Project Manager'), s('Active', className: 'rounded text-emerald'), s('24 Jun 2024'), s('Enabled', className: 'rounded text-amber'), s(''),
        s('Noah Anderson'), s('anderson@example.com'), s('UX Designer'),     s('Active', className: 'rounded text-emerald'), s('15 Mar 2023'), s('Enabled', className: 'rounded text-amber'), s(''),
      ],
      '04': [
        s('Darnell Steward'), s('(+33) 77 55 87 24'), s('USA'),    s('\$167.77'),   s('0'), s('May 9, 2014'),    s('Active', className: 'rounded text-amber'), s(''),
        s('Ralph Edwards'),   s('(+33) 66 55 51 05'), s('Canada'), s('\$5,872.78'), s('5'), s('April 28, 2016'), s('Failed', className: 'rounded text-rose'),  s(''),
      ],
      '05': [
        s('Nick Johnston'), s('4A · 1A', className: 'rounded text-emerald'), s('Company'),    s('2664 7548 6521'), s('0% / 0%'),  s('0% / 0%'),
        s('Alice Smith'),   s('5B · 2A', className: 'rounded text-emerald'), s('Individual'), s('1234 5678 9101'), s('10% / 5%'), s('5% / 2%'),
      ],
    };
    return map[v] ?? map['01']!;
  }

  @override
  Widget build(BuildContext context) {
    final cols = widget.columns ?? _fallbackCols(widget.variant);
    final flatRows = widget.rows ?? _fallbackRows(widget.variant);
    final n = cols.isEmpty ? 1 : cols.length;

    final allChunked = <List<TextSegment>>[];
    for (var i = 0; i < flatRows.length; i += n) {
      allChunked.add(flatRows.sublist(i, (i + n).clamp(0, flatRows.length)));
    }

    final colIdx = cols.length > 1 ? 1 : 0;
    final filterOptions = <String>{};
    for (final row in allChunked) {
      if (colIdx < row.length && row[colIdx].text.trim().isNotEmpty) {
        filterOptions.add(row[colIdx].text.trim());
      }
    }

    var rows2d = _sortAsc ? [...allChunked] : allChunked.reversed.toList();
    if (_categoryFilter != 'all') {
      rows2d = rows2d.where((r) => colIdx < r.length && r[colIdx].text == _categoryFilter).toList();
    }
    if (_search.isNotEmpty) {
      final q = _search.toLowerCase();
      rows2d = rows2d.where((row) => row.any((c) => c.text.toLowerCase().contains(q))).toList();
    }

    return Container(
      decoration: BoxDecoration(
        color: _cardBg,
        border: Border.all(color: _border),
        borderRadius: BorderRadius.circular(12),
      ),
      clipBehavior: Clip.antiAlias,
      child: Column(crossAxisAlignment: CrossAxisAlignment.stretch, children: [
        if (widget.title != null)
          Padding(
            padding: const EdgeInsets.fromLTRB(16, 16, 16, 0),
            child: Text(widget.title!, style: const TextStyle(fontWeight: FontWeight.w600, color: _zinc100)),
          ),

        DataTableFilterWidget(
          props: DataTableFilterProps(
            columns: cols,
            filterOptions: filterOptions.toList(),
            searchQuery: _search,
            categoryFilter: _categoryFilter,
            hiddenColumns: _hiddenCols,
            onSearchChange: (val) => setState(() => _search = val),
            onCategoryChange: (val) => setState(() => _categoryFilter = val),
            onToggleColumn: (ci) => setState(() {
              if (ci == 0) return;
              _hiddenCols.contains(ci) ? _hiddenCols.remove(ci) : _hiddenCols.add(ci);
            }),
            onReset: () => setState(() {
              _search = '';
              _categoryFilter = 'all';
              _hiddenCols.clear();
            }),
          ),
        ),

        SingleChildScrollView(
          scrollDirection: Axis.horizontal,
          child: DataTable(
            headingRowColor: const WidgetStatePropertyAll(_baseBg),
            dataRowColor: WidgetStateProperty.resolveWith(
              (states) => states.contains(WidgetState.hovered) ? _hover : Colors.transparent,
            ),
            dividerThickness: 0.5,
            border: const TableBorder(
              horizontalInside: BorderSide(color: _border, width: 0.5),
              bottom: BorderSide(color: _border, width: 0.5),
            ),
            sortColumnIndex: 0,
            sortAscending: _sortAsc,
            onSelectAll: (_) {},
            columns: cols
                .asMap()
                .entries
                .where((e) => !_hiddenCols.contains(e.key))
                .map(
                  (e) => DataColumn(
                    label: Text(
                      e.value.text,
                      style: const TextStyle(fontSize: 11, color: _zinc500, fontWeight: FontWeight.w600),
                    ),
                    onSort: (_, __) => setState(() => _sortAsc = !_sortAsc),
                  ),
                )
                .toList(),
            rows: rows2d.asMap().entries.map((entry) {
              final ri = entry.key;
              final row = entry.value;
              return DataRow(
                selected: _selected.contains(ri),
                onSelectChanged: (val) =>
                    setState(() => val == true ? _selected.add(ri) : _selected.remove(ri)),
                cells: row
                    .asMap()
                    .entries
                    .where((e) => !_hiddenCols.contains(e.key))
                    .map((e) => DataCell(TextWidget(text: e.value.text, className: e.value.className)))
                    .toList(),
              );
            }).toList(),
          ),
        ),

        if (widget.variant != '01' && widget.variant != '05')
          Container(
            decoration: const BoxDecoration(border: Border(top: BorderSide(color: _border))),
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
            child: Row(mainAxisAlignment: MainAxisAlignment.spaceBetween, children: [
              Text(
                widget.variant == '02' ? 'Page 1 of 13' : 'Page 1 of 8',
                style: const TextStyle(fontSize: 11, color: _zinc500),
              ),
              Row(children: [
                _pageBtn('‹', () {}),
                const SizedBox(width: 6),
                _pageBtn('›', () {}),
              ]),
            ]),
          ),
      ]),
    );
  }

  Widget _pageBtn(String label, VoidCallback onTap) {
    return GestureDetector(
      onTap: onTap,
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
        decoration: BoxDecoration(
          color: _surface,
          border: Border.all(color: _border),
          borderRadius: BorderRadius.circular(4),
        ),
        child: Text(label, style: const TextStyle(fontSize: 13, color: _zinc300)),
      ),
    );
  }
}
