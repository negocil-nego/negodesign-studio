import 'package:flutter/material.dart';
import 'types.dart';

class InputTimeViewModel extends ChangeNotifier {
  final InputTimeProps props;
  final ValueChanged<String>? onSelect;

  late String value;
  String hour = '02';
  String minute = '30';
  String period = 'AM';

  static final List<String> hours = List.generate(12, (i) => (i + 1).toString().padLeft(2, '0'));
  static final List<String> minutes = List.generate(12, (i) => (i * 5).toString().padLeft(2, '0'));
  static const List<String> periods = ['AM', 'PM'];

  InputTimeViewModel(this.props, {this.onSelect}) {
    value = props.inputProps.value ?? '';
    if (value.isNotEmpty) {
      final match = RegExp(r'(\d{2}):(\d{2})\s?(AM|PM)', caseSensitive: false).firstMatch(value);
      if (match != null) {
        hour = match.group(1)!;
        minute = match.group(2)!;
        period = match.group(3)!.toUpperCase();
      }
    }
  }

  String get displayTime => hour.isNotEmpty && minute.isNotEmpty ? '$hour:$minute $period' : '-- : --';

  void applySelection(Map<String, String>? result) {
    if (result == null) return;
    hour = result['hour']!;
    minute = result['minute']!;
    period = result['period']!;
    value = '$hour:$minute $period';
    onSelect?.call(value);
    notifyListeners();
  }

  void clearTime() {
    value = '';
    hour = '';
    minute = '';
    period = 'AM';
    onSelect?.call('');
    notifyListeners();
  }
}
