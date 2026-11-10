import 'package:flutter/material.dart';
import 'types.dart';

class InputDateViewModel extends ChangeNotifier {
  final InputDateProps props;
  final ValueChanged<String>? onSelect;

  late String value;
  int day = 21;
  int month = 10;
  int year = 2023;

  static const List<String> months = [
    'January', 'February', 'March', 'April', 'May', 'June',
    'July', 'August', 'September', 'October', 'November', 'December',
  ];

  InputDateViewModel(this.props, {this.onSelect}) {
    value = props.inputProps.value ?? 'November 21, 2023';
  }

  void applySelection(Map<String, int>? result) {
    if (result == null) return;
    day = result['day']!;
    month = result['month']!;
    year = result['year']!;
    value = '${months[month]} $day, $year';
    onSelect?.call(value);
    notifyListeners();
  }
}

class CalendarDialogViewModel extends ChangeNotifier {
  int selectedDay;
  int month;
  int year;
  final List<String> months;

  CalendarDialogViewModel({
    required this.selectedDay,
    required this.month,
    required this.year,
    required this.months,
  });

  String get headerTitle => '${months[month]} $year';

  List<int> get paddingDays {
    final startWeekday = DateTime(year, month, 1).weekday - 1;
    final prevMonthDays = DateTime(year, month, 0).day;
    return List.generate(startWeekday, (i) => prevMonthDays - startWeekday + i + 1);
  }

  List<int> get days {
    final daysInMonth = DateTime(year, month + 1, 0).day;
    return List.generate(daysInMonth, (i) => i + 1);
  }

  void changeMonth(int delta) {
    month += delta;
    if (month > 11) {
      month = 0;
      year++;
    } else if (month < 0) {
      month = 11;
      year--;
    }
    notifyListeners();
  }
}
