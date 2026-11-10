import 'package:flutter/material.dart';
import '../../data/viewmodel.dart';

class CalendarDialog extends StatefulWidget {
  final int day;
  final int month;
  final int year;
  final List<String> months;

  const CalendarDialog({
    super.key,
    required this.day,
    required this.month,
    required this.year,
    required this.months,
  });

  @override
  State<CalendarDialog> createState() => _CalendarDialogState();
}

class _CalendarDialogState extends State<CalendarDialog> {
  late final CalendarDialogViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = CalendarDialogViewModel(selectedDay: widget.day, month: widget.month, year: widget.year, months: widget.months);
  }

  @override
  void dispose() {
    _vm.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: _vm,
      builder: (context, _) {
        final pad = _vm.paddingDays;
        final days = _vm.days;
        return AlertDialog(
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
          content: SizedBox(
            width: 300,
            height: 360,
            child: Column(children: [
              Row(mainAxisAlignment: MainAxisAlignment.spaceBetween, children: [
                IconButton(icon: const Icon(Icons.chevron_left, size: 18), onPressed: () => _vm.changeMonth(-1)),
                Text(_vm.headerTitle, style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 14)),
                IconButton(icon: const Icon(Icons.chevron_right, size: 18), onPressed: () => _vm.changeMonth(1)),
              ]),
              const SizedBox(height: 8),
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceAround,
                children: const ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
                    .map((d) => Text(d, style: const TextStyle(fontSize: 11, color: Colors.grey)))
                    .toList(),
              ),
              const SizedBox(height: 12),
              Expanded(
                child: GridView.builder(
                  gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(crossAxisCount: 7),
                  itemCount: pad.length + days.length,
                  itemBuilder: (context, index) {
                    if (index < pad.length) {
                      return Center(child: Text('${pad[index]}', style: TextStyle(color: Colors.grey.withValues(alpha: 0.4))));
                    }
                    final d = days[index - pad.length];
                    final isSelected = d == _vm.selectedDay;
                    return GestureDetector(
                      onTap: () => Navigator.of(context).pop({'day': d, 'month': _vm.month, 'year': _vm.year}),
                      child: Container(
                        margin: const EdgeInsets.all(4),
                        decoration: BoxDecoration(color: isSelected ? Colors.orange : Colors.transparent, shape: BoxShape.circle),
                        child: Center(
                          child: Text('$d', style: TextStyle(color: isSelected ? Colors.white : null, fontWeight: isSelected ? FontWeight.bold : FontWeight.normal)),
                        ),
                      ),
                    );
                  },
                ),
              ),
            ]),
          ),
        );
      },
    );
  }
}
