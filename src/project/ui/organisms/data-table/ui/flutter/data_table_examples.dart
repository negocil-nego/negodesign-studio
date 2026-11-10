import '../../data/types.dart';

class DataTableExamplesFlutter {
  const DataTableExamplesFlutter._();

  static List<TextSegment> columns(String v) {
    const map = {
      '01': ['Name', 'Role', 'Experience', 'Company', 'Score', 'Date', 'Action'],
      '02': ['Shipment ID', 'Description', 'Location', 'Type', 'Priority', 'Carrier', 'Contact'],
      '03': ['Full name', 'Email', 'Role', 'Status', 'Joined date', '2F Auth', 'Actions'],
      '04': ['Customer', 'Phone', 'Country', 'Balance', 'Total invoice', 'Created On', 'Status', 'Action'],
      '05': ['Name', 'Trading Zones', 'Type', 'ACN', 'Commission % (Buy/Sell)', 'Commission % (Buy/Lease)'],
    };
    return (map[v] ?? map['01']!).map((l) => TextSegment(text: l)).toList();
  }

  static List<TextSegment> rows(String v) {
    TextSegment s(String t, [String? c]) => TextSegment(text: t, className: c);
    final map = <String, List<TextSegment>>{
      '01': [
        s('Maya Thompson'), s('Whole seller'), s('3 years'), s('ACE Homes LLC +2'), s('12,054'), s('12/24/2024'), s(''),
        s('Ethan Carter'), s('Property manager'), s('3 years'), s('A&H Realty'), s('12,054'), s('12/24/2024'), s(''),
      ],
      '02': [
        s('#SHP-842'), s('Pallet of Electronics'), s('Zone A'), s('Outbound'), s('Urgent', 'rounded text-red'), s('FedEx'), s('dispatch@fedex.com'),
        s('#SHP-841'), s('Medical Supplies'), s('Cold B'), s('Inbound'), s('Urgent', 'rounded text-red'), s('DHL'), s('med@dhl.com'),
      ],
      '03': [
        s('Liam Smith'), s('smith@example.com'), s('Project Manager'), s('Active', 'rounded text-emerald'), s('24 Jun 2024'), s('Enabled', 'rounded text-amber'), s(''),
        s('Noah Anderson'), s('anderson@example.com'), s('UX Designer'), s('Active', 'rounded text-emerald'), s('15 Mar 2023'), s('Enabled', 'rounded text-amber'), s(''),
      ],
      '04': [
        s('Darnell Steward'), s('(+33) 77 55 87 24'), s('USA'), s(r'$167.77'), s('0'), s('May 9, 2014'), s('Active', 'rounded text-amber'), s(''),
        s('Ralph Edwards'), s('(+33) 66 55 51 05'), s('Canada'), s(r'$5,872.78'), s('5'), s('April 28, 2016'), s('Failed', 'rounded text-rose'), s(''),
      ],
      '05': [
        s('Nick Johnston'), s('4A · 1A', 'rounded text-emerald'), s('Company'), s('2664 7548 6521'), s('0% / 0%'), s('0% / 0%'),
        s('Alice Smith'), s('5B · 2A', 'rounded text-emerald'), s('Individual'), s('1234 5678 9101'), s('10% / 5%'), s('5% / 2%'),
      ],
    };
    return map[v] ?? map['01']!;
  }
}
