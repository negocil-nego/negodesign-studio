import '../../data/types.dart';

class DataTableExamplesFlutter {
  const DataTableExamplesFlutter._();

  static List<TextSegment> columns(String v) {
    const map = {
      '01': ['Name', 'Role', 'Experience', 'Company', 'Score', 'Status', 'Action'],
      '02': ['Shipment ID', 'Description', 'Location', 'Type', 'Priority', 'Carrier', 'Contact'],
      '03': ['Full name', 'Email', 'Role', 'Status', 'Joined date', '2F Auth', 'Actions'],
      '04': ['Customer', 'Phone', 'Country', 'Balance', 'Total invoice', 'Created On', 'Status', 'Action'],
      '05': ['Name', 'Trading Zones', 'Type', 'ACN', 'Commission % (Buy/Sell)', 'Commission % (Buy/Lease)'],
    };
    final cols = map[v] ?? map['01']!;
    return [
      for (var i = 0; i < cols.length; i++)
        TextSegment(
          text: cols[i],
          type: (cols[i] == 'Action' || cols[i] == 'Actions') ? 'dropdown' : 'text',
        ),
    ];
  }

  static List<TextSegment> rows(String v) {
    TextSegment s(String t, [String? c]) => TextSegment(text: t, className: c);
    TextSegment b(String t, String variant, [bool dot = true]) =>
        TextSegment(text: t, type: 'badge', badgeVariant: variant, dot: dot);
    TextSegment d() => const TextSegment(text: '', type: 'dropdown', actions: ['Edit', 'Details', 'Delete']);

    final map = <String, List<TextSegment>>{
      '01': [
        s('Maya Thompson'), b('Whole seller', 'primary', false), s('3 years'), s('ACE Homes LLC +2'), s('12,054'), b('Active', 'success'), d(),
        s('Ethan Carter'), b('Property manager', 'info', false), s('3 years'), s('A&H Realty'), s('12,054'), b('Active', 'success'), d(),
      ],
      '02': [
        s('#SHP-842'), s('Pallet of Electronics'), s('Zone A'), s('Outbound'), b('Urgent', 'danger'), s('FedEx'), s('dispatch@fedex.com'),
        s('#SHP-841'), s('Medical Supplies'), s('Cold B'), s('Inbound'), b('Urgent', 'danger'), s('DHL'), s('med@dhl.com'),
      ],
      '03': [
        s('Liam Smith'), s('smith@example.com'), s('Project Manager'), b('Active', 'success'), s('24 Jun 2024'), b('Enabled', 'warning', false), d(),
        s('Noah Anderson'), s('anderson@example.com'), s('UX Designer'), b('Active', 'success'), s('15 Mar 2023'), b('Enabled', 'warning', false), d(),
      ],
      '04': [
        s('Darnell Steward'), s('(+33) 77 55 87 24'), s('USA'), s(r'$167.77'), s('0'), s('May 9, 2014'), b('Active', 'success'), d(),
        s('Ralph Edwards'), s('(+33) 66 55 51 05'), s('Canada'), s(r'$5,872.78'), s('5'), s('April 28, 2016'), b('Failed', 'danger'), d(),
      ],
      '05': [
        s('Nick Johnston'), b('4A · 1A', 'success', false), s('Company'), s('2664 7548 6521'), s('0% / 0%'), s('0% / 0%'),
        s('Alice Smith'), b('5B · 2A', 'success', false), s('Individual'), s('1234 5678 9101'), s('10% / 5%'), s('5% / 2%'),
      ],
    };
    return map[v] ?? map['01']!;
  }
}
