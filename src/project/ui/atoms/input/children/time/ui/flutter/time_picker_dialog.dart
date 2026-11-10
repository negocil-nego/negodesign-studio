import 'package:flutter/material.dart';
import '../../data/viewmodel.dart';

class TimePickerDialog extends StatelessWidget {
  final String hour;
  final String minute;
  final String period;

  const TimePickerDialog({
    super.key,
    required this.hour,
    required this.minute,
    required this.period,
  });

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      backgroundColor: const Color(0xFF1C1C1E),
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16), side: const BorderSide(color: Color(0xFF2C2C2E))),
      content: SizedBox(
        width: 240,
        height: 240,
        child: Row(
          children: [
            TimeWheelColumn(
              items: InputTimeViewModel.hours,
              selectedValue: hour,
              onSelected: (h) => Navigator.of(context).pop({'hour': h, 'minute': minute, 'period': period}),
            ),
            const VerticalDivider(color: Color(0xFF2C2C2E), width: 9),
            TimeWheelColumn(
              items: InputTimeViewModel.minutes,
              selectedValue: minute,
              onSelected: (m) => Navigator.of(context).pop({'hour': hour, 'minute': m, 'period': period}),
            ),
            const VerticalDivider(color: Color(0xFF2C2C2E), width: 9),
            TimeWheelColumn(
              items: InputTimeViewModel.periods,
              selectedValue: period,
              onSelected: (p) => Navigator.of(context).pop({'hour': hour, 'minute': minute, 'period': p}),
            ),
          ],
        ),
      ),
    );
  }
}

class TimeWheelColumn extends StatelessWidget {
  final List<String> items;
  final String selectedValue;
  final ValueChanged<String> onSelected;

  const TimeWheelColumn({
    super.key,
    required this.items,
    required this.selectedValue,
    required this.onSelected,
  });

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: ListWheelScrollView.useDelegate(
        itemExtent: 40,
        physics: const FixedExtentScrollPhysics(),
        onSelectedItemChanged: (index) => onSelected(items[index]),
        childDelegate: ListWheelChildBuilderDelegate(
          childCount: items.length,
          builder: (context, index) {
            final item = items[index];
            final isSelected = item == selectedValue;
            return Center(
              child: Container(
                width: 50,
                height: 32,
                decoration: BoxDecoration(color: isSelected ? Colors.blue : Colors.transparent, borderRadius: BorderRadius.circular(8)),
                child: Center(
                  child: Text(
                    item,
                    style: TextStyle(color: isSelected ? Colors.white : Colors.grey, fontWeight: isSelected ? FontWeight.bold : FontWeight.normal, fontSize: 14),
                  ),
                ),
              ),
            );
          },
        ),
      ),
    );
  }
}
