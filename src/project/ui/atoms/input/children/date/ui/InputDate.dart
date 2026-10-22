import 'package:flutter/material.dart';
import '../data/types.dart';

class InputDate extends StatefulWidget {
  final InputDateProps props;
  final ValueChanged<String>? onSelect;

  const InputDate(this.props, {this.onSelect, super.key});

  @override
  State<InputDate> createState() => _InputDateState();
}

class _InputDateState extends State<InputDate> {
  late String _value;
  int _day = 21;
  int _month = 10;
  int _year = 2023;

  final List<String> _months = const [
    'January',
    'February',
    'March',
    'April',
    'May',
    'June',
    'July',
    'August',
    'September',
    'October',
    'November',
    'December',
  ];

  @override
  void initState() {
    super.initState();
    _value = widget.props.inputProps.value ?? 'November 21, 2023';
  }

  Future<void> _openPicker() async {
    final result = await showDialog<Map<String, int>>(
      context: context,
      builder:
          (context) => _CalendarDialog(
            day: _day,
            month: _month,
            year: _year,
            months: _months,
          ),
    );
    if (result != null) {
      setState(() {
        _day = result['day']!;
        _month = result['month']!;
        _year = result['year']!;
        _value = '${_months[_month]} $_day, $_year';
      });
      widget.onSelect?.call(_value);
    }
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: double.infinity,
      child: InkWell(
        onTap: widget.props.inputProps.disabled ? null : _openPicker,
        borderRadius: BorderRadius.circular(8),
        child: Container(
          width: double.infinity,
          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
          decoration: BoxDecoration(
            color: Theme.of(context).cardColor,
            borderRadius: BorderRadius.circular(8),
            border: Border.all(color: Colors.orange.withOpacity(0.5)),
          ),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    if (widget.props.labelProps != null)
                      Text(
                        widget.props.labelProps!.label.toUpperCase(),
                        style: const TextStyle(
                          fontSize: 10,
                          color: Colors.grey,
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                    const SizedBox(height: 2),
                    Row(
                      children: [
                        const Icon(
                          Icons.label_outline,
                          size: 16,
                          color: Colors.grey,
                        ),
                        const SizedBox(width: 6),
                        Text(
                          _value,
                          style: const TextStyle(
                            fontSize: 14,
                            fontWeight: FontWeight.w500,
                          ),
                          overflow: TextOverflow.ellipsis,
                        ),
                      ],
                    ),
                  ],
                ),
              ),
              const Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Icon(Icons.help_outline, size: 14, color: Colors.grey),
                  Icon(Icons.help_outline, size: 14, color: Colors.grey),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _CalendarDialog extends StatefulWidget {
  final int day;
  final int month;
  final int year;
  final List<String> months;

  const _CalendarDialog({
    required this.day,
    required this.month,
    required this.year,
    required this.months,
  });

  @override
  State<_CalendarDialog> createState() => _CalendarDialogState();
}

class _CalendarDialogState extends State<_CalendarDialog> {
  late int _selectedDay = widget.day;
  late int _month = widget.month;
  late int _year = widget.year;

  void _changeMonth(int delta) {
    setState(() {
      _month += delta;
      if (_month > 11) {
        _month = 0;
        _year++;
      } else if (_month < 0) {
        _month = 11;
        _year--;
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    final daysInMonth = DateTime(_year, _month + 1, 0).day;
    final startWeekday = (DateTime(_year, _month, 1).weekday - 1);
    final prevMonthDays = DateTime(_year, _month, 0).day;

    final paddingDays = List.generate(
      startWeekday,
      (i) => prevMonthDays - startWeekday + i + 1,
    );
    final days = List.generate(daysInMonth, (i) => i + 1);

    return AlertDialog(
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      content: SizedBox(
        width: 300,
        height: 360,
        child: Column(
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                IconButton(
                  icon: const Icon(Icons.help_outline, size: 18),
                  onPressed: () => _changeMonth(-1),
                ),
                Text(
                  '${widget.months[_month]} $_year',
                  style: const TextStyle(
                    fontWeight: FontWeight.bold,
                    fontSize: 14,
                  ),
                ),
                IconButton(
                  icon: const Icon(Icons.help_outline, size: 18),
                  onPressed: () => _changeMonth(1),
                ),
              ],
            ),
            const SizedBox(height: 8),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceAround,
              children:
                  const ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
                      .map(
                        (d) => Text(
                          d,
                          style: const TextStyle(
                            fontSize: 11,
                            color: Colors.grey,
                          ),
                        ),
                      )
                      .toList(),
            ),
            const SizedBox(height: 12),
            Expanded(
              child: GridView.builder(
                gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
                  crossAxisCount: 7,
                ),
                itemCount: paddingDays.length + days.length,
                itemBuilder: (context, index) {
                  if (index < paddingDays.length) {
                    return Center(
                      child: Text(
                        '${paddingDays[index]}',
                        style: const TextStyle(
                          color: Colors.grey,
                          opacity: 0.4,
                        ),
                      ),
                    );
                  }
                  final d = days[index - paddingDays.length];
                  final isSelected = d == _selectedDay;
                  return GestureDetector(
                    onTap:
                        () => Navigator.of(
                          context,
                        ).pop({'day': d, 'month': _month, 'year': _year}),
                    child: Container(
                      margin: const EdgeInsets.all(4),
                      decoration: BoxDecoration(
                        color: isSelected ? Colors.orange : Colors.transparent,
                        shape: BoxShape.circle,
                      ),
                      child: Center(
                        child: Text(
                          '$d',
                          style: TextStyle(
                            color: isSelected ? Colors.white : null,
                            fontWeight:
                                isSelected
                                    ? FontWeight.bold
                                    : FontWeight.normal,
                          ),
                        ),
                      ),
                    ),
                  );
                },
              ),
            ),
          ],
        ),
      ),
    );
  }
}
