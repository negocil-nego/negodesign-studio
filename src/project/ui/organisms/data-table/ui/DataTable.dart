import 'package:flutter/material.dart';
import '../data/types.dart';

typedef TextProps = TextSegment;

/// Widget equivalent to <Text> atom.
class TextWidget extends StatelessWidget {
  const TextWidget({super.key, required this.text, this.className});
  final String text;
  final String? className;

  @override
  Widget build(BuildContext context) {
    Color? color;
    if (className != null) {
      if (className!.contains('text-red'))     color = const Color(0xFFDC2626);
      if (className!.contains('text-orange'))  color = const Color(0xFFEA580C);
      if (className!.contains('text-emerald')) color = const Color(0xFF059669);
      if (className!.contains('text-amber'))   color = const Color(0xFFB45309);
      if (className!.contains('text-rose'))    color = const Color(0xFFE11D48);
    }
    final hasBg = className != null &&
        (className!.contains('bg-') || className!.contains('rounded'));
    if (hasBg) {
      return Container(
        padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
        decoration: BoxDecoration(
          color: color?.withOpacity(0.1) ?? Colors.grey.shade100,
          borderRadius: BorderRadius.circular(4),
        ),
        child: Text(text, style: TextStyle(fontSize: 11, color: color)),
      );
    }
    return Text(text, style: TextStyle(fontSize: 12, color: color));
  }
}

class DataTableWidget extends StatefulWidget {
  const DataTableWidget({
    super.key,
    this.variant = '01',
    this.title,
    this.columns,
    this.rows,
  });

  final String variant;
  final String? title;
  /// Flat list of column headers (TextSegment).
  final List<TextSegment>? columns;
  /// Flat list of all cell values — chunked by columns.length inside the widget.
  final List<TextSegment>? rows;

  @override
  State<DataTableWidget> createState() => _DataTableWidgetState();
}

class _DataTableWidgetState extends State<DataTableWidget> {
  bool _sortAsc = true;
  final Set<int> _selected = {};

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
    const badge = 'rounded bg-amber-100 text-amber-900';
    const danger = 'rounded bg-rose-100 text-rose-700';
    const green  = 'rounded-full bg-emerald-100 text-emerald-700';
    final map = <String, List<TextSegment>>{
      '01': [
        s('Maya Thompson'),  s('Whole seller'),     s('3 years'), s('ACE Homes LLC  +2'), s('12,054'), s('12/24/2024'), s(''),
        s('Ethan Carter'),   s('Property manager'), s('3 years'), s('A&H Realty'),        s('12,054'), s('12/24/2024'), s(''),
      ],
      '02': [
        s('#SHP-842'), s('Pallet of Electronics'), s('Zone A'), s('Outbound'), s('Urgent', className: 'rounded bg-red-50 text-red-600'),   s('FedEx Freight'), s('dispatch@fedex.com'),
        s('#SHP-841'), s('Medical Supplies'),       s('Cold B'), s('Inbound'),  s('Urgent', className: 'rounded bg-red-50 text-red-600'),   s('DHL Express'),   s('med@dhl.com'),
      ],
      '03': [
        s('Liam Smith'),    s('smith@example.com'),    s('Project Manager'), s('Active',   className: badge), s('24 Jun 2024'), s('Enabled', className: badge), s(''),
        s('Noah Anderson'), s('anderson@example.com'), s('UX Designer'),     s('Active',   className: badge), s('15 Mar 2023'), s('Enabled', className: badge), s(''),
      ],
      '04': [
        s('Darnell Steward'), s('(+33) 77 55 87 24'), s('USA'),    s('\$167.77'),   s('0'), s('May 9, 2014'),     s('Active', className: badge), s(''),
        s('Ralph Edwards'),   s('(+33) 66 55 51 05'), s('Canada'), s('\$5,872.78'), s('5'), s('April 28, 2016'),  s('Failed', className: danger), s(''),
      ],
      '05': [
        s('Nick Johnston'), s('4A · 1A', className: green), s('Company'),    s('2664 7548 6521'), s('0% / 0%'),  s('0% / 0%'),
        s('Alice Smith'),   s('5B · 2A', className: green), s('Individual'), s('1234 5678 9101'), s('10% / 5%'), s('5% / 2%'),
      ],
    };
    return map[v] ?? map['01']!;
  }

  @override
  Widget build(BuildContext context) {
    final cols = widget.columns ?? _fallbackCols(widget.variant);
    final flatRows = widget.rows ?? _fallbackRows(widget.variant);
    final n = cols.isEmpty ? 1 : cols.length;

    // Chunk flat rows into 2-D
    var rows2d = <List<TextSegment>>[];
    for (var i = 0; i < flatRows.length; i += n) {
      rows2d.add(flatRows.sublist(i, (i + n).clamp(0, flatRows.length)));
    }
    if (!_sortAsc) rows2d = rows2d.reversed.toList();

    return Card(
      clipBehavior: Clip.antiAlias,
      color: Colors.white,
      child: Column(crossAxisAlignment: CrossAxisAlignment.stretch, children: [
        if (widget.title != null)
          Padding(padding: const EdgeInsets.all(16),
              child: Text(widget.title!, style: const TextStyle(fontWeight: FontWeight.w600))),
        SingleChildScrollView(
          scrollDirection: Axis.horizontal,
          child: DataTable(
            headingRowColor: WidgetStatePropertyAll(Colors.grey.shade50),
            sortColumnIndex: 0,
            sortAscending: _sortAsc,
            onSelectAll: (_) {},
            columns: cols.map((col) => DataColumn(
              label: TextWidget(text: col.text, className: col.className),
              onSort: (_, __) => setState(() => _sortAsc = !_sortAsc),
            )).toList(),
            rows: rows2d.asMap().entries.map((entry) {
              final ri = entry.key;
              final row = entry.value;
              return DataRow(
                selected: _selected.contains(ri),
                onSelectChanged: (val) => setState(() {
                  val == true ? _selected.add(ri) : _selected.remove(ri);
                }),
                cells: row.map((seg) => DataCell(
                  TextWidget(text: seg.text, className: seg.className),
                )).toList(),
              );
            }).toList(),
          ),
        ),
        if (widget.variant != '01' && widget.variant != '05')
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
            child: Row(mainAxisAlignment: MainAxisAlignment.spaceBetween, children: [
              const Text('Page 1 of 13', style: TextStyle(fontSize: 11, color: Colors.grey)),
              Row(children: [
                OutlinedButton(onPressed: () {}, child: const Text('‹')),
                const SizedBox(width: 8),
                OutlinedButton(onPressed: () {}, child: const Text('›')),
              ]),
            ]),
          ),
      ]),
    );
  }
}
