import 'package:flutter/material.dart';
import '../data/types.dart';

class DataTableWidget extends StatelessWidget {
  const DataTableWidget({super.key, this.variant = '01', this.title, this.columns, this.rows});
  final String variant;
  final String? title;
  final List<DataTableColumn>? columns;
  final List<DataTableRowData>? rows;

  static const headers = {
    '01': ['Alex', 'Role', 'Experience', 'Company', 'Score', 'Date', 'Action'],
    '02': ['Shipment ID', 'Description', 'Location', 'Type', 'Priority', 'Carrier', 'Contact'],
    '03': ['Full name', 'Email', 'Role', 'Status', 'Joined date', '2F Auth', 'Actions'],
    '04': ['Customer', 'Phone', 'Country', 'Balance', 'Total invoice', 'Created On', 'Status'],
    '05': ['Name', 'Trading Zones', 'Type', 'ACN', 'Commission % (Buying / Selling)', 'Commission % (Buying / Leasing)'],
  };
  static const rows = {
    '01': [
      ['Maya Thompson', 'Whole seller', '3 years', 'ACE Homes LLC', '12,054', '12/24/2024', '•••'],
      ['Ethan Carter', 'Property manager', '3 years', 'A&H Realty', '12,054', '12/24/2024', '•••'],
    ],
    '02': [
      ['#SHP-842', 'Pallet of Electronics (45 units)', 'Zone A - Dock 4', 'Outbound', 'Urgent', 'FedEx Freight', 'dispatch@fedex.com'],
      ['#SHP-841', 'Medical Supplies Restock', 'Cold Storage B', 'Inbound', 'Urgent', 'DHL Express', 'med@dhl.com'],
    ],
    '03': [
      ['Liam Smith', 'smith@example.com', 'Project Manager', 'Active', '24 Jun 2024', 'Enabled', 'Edit / Delete'],
      ['Noah Anderson', 'anderson@example.com', 'UX Designer', 'Active', '15 Mar 2023', 'Enabled', 'Edit / Delete'],
    ],
    '04': [
      ['Darnell Steward', '(+33) 77 55 87 24', '🇺🇸 USA', '\$167.77', '0', 'May 9, 2014', 'Active'],
      ['Ralph Edwards', '(+33) 66 55 51 05 09', '🇨🇦 Canada', '\$5,872.78', '5', 'April 28, 2016', 'Failed'],
    ],
    '05': [
      ['Nick Johnston', '4A · 1A', 'Company', '2664 7548 6521', '0% / 0%', '0% / 0%'],
      ['Alice Smith', '5B · 2A', 'Individual', '1234 5678 9101', '10% / 5%', '5% / 2%'],
    ],
  };

  @override
  Widget build(BuildContext context) {
    final cols = columns ?? (headers[variant] ?? headers['01']!).map((label) => DataTableColumn(label: label, key: label.toLowerCase())).toList();
    final data = rows ?? (fallbackRows[variant] ?? fallbackRows['01']!).asMap().entries.map((entry) => DataTableRowData(id: '$variant-${entry.key}', cells: entry.value)).toList();
    return Card(
      clipBehavior: Clip.antiAlias,
      color: Colors.white,
      child: Column(crossAxisAlignment: CrossAxisAlignment.stretch, children: [
        if (title != null) Padding(padding: const EdgeInsets.all(16), child: Text(title!, style: const TextStyle(fontWeight: FontWeight.w600))),
        SingleChildScrollView(scrollDirection: Axis.horizontal, child: DataTable(
          headingRowColor: WidgetStatePropertyAll(Colors.grey.shade50),
          columns: cols.map((column) => DataColumn(label: Text(column.label, style: const TextStyle(fontSize: 11)))).toList(),
          rows: data.map((row) => DataRow(cells: row.cells.map((value) => DataCell(Text(value, style: const TextStyle(fontSize: 12)))).toList())).toList(),
        )),
        if (variant != '01' && variant != '05') const Padding(padding: EdgeInsets.all(12), child: Text('Page 1 of 13')),
      ]),
    );
  }

  static const fallbackRows = {
    '01': [
      ['Maya Thompson', 'Whole seller', '3 years', 'ACE Homes LLC', '12,054', '12/24/2024', '•••'],
      ['Ethan Carter', 'Property manager', '3 years', 'A&H Realty', '12,054', '12/24/2024', '•••'],
    ],
    '02': [
      ['#SHP-842', 'Pallet of Electronics (45 units)', 'Zone A - Dock 4', 'Outbound', 'Urgent', 'FedEx Freight', 'dispatch@fedex.com'],
      ['#SHP-841', 'Medical Supplies Restock', 'Cold Storage B', 'Inbound', 'Urgent', 'DHL Express', 'med@dhl.com'],
    ],
    '03': [
      ['Liam Smith', 'smith@example.com', 'Project Manager', 'Active', '24 Jun 2024', 'Enabled', 'Edit / Delete'],
      ['Noah Anderson', 'anderson@example.com', 'UX Designer', 'Active', '15 Mar 2023', 'Enabled', 'Edit / Delete'],
    ],
    '04': [
      ['Darnell Steward', '(+33) 77 55 87 24', '🇺🇸 USA', '\$167.77', '0', 'May 9, 2014', 'Active', ''],
      ['Ralph Edwards', '(+33) 66 55 51 05 09', '🇨🇦 Canada', '\$5,872.78', '5', 'April 28, 2016', 'Failed', ''],
    ],
    '05': [
      ['Nick Johnston', '4A · 1A', 'Company', '2664 7548 6521', '0% / 0%', '0% / 0%'],
      ['Alice Smith', '5B · 2A', 'Individual', '1234 5678 9101', '10% / 5%', '5% / 2%'],
    ],
  };
}
