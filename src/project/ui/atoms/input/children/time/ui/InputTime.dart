import 'package:flutter/material.dart';
import '../data/types.dart';

class InputTime extends StatefulWidget {
  final InputTimeProps props;
  final ValueChanged<String>? onSelect;

  const InputTime(this.props, {this.onSelect, super.key});

  @override
  State<InputTime> createState() => _InputTimeState();
}

class _InputTimeState extends State<InputTime> {
  late String _value;
  String _hour = "02";
  String _minute = "30";
  String _period = "AM";

  @override
  void initState() {
    super.initState();
    _value = widget.props.inputProps.value ?? '';
    if (_value.isNotEmpty) {
      final match = RegExp(r'(\d{2}):(\d{2})\s?(AM|PM)', caseSensitive: false)
          .firstMatch(_value);
      if (match != null) {
        _hour = match.group(1)!;
        _minute = match.group(2)!;
        _period = match.group(3)!.toUpperCase();
      }
    }
  }

  String get _displayTime =>
      _hour.isNotEmpty && _minute.isNotEmpty ? "$_hour:$_minute $_period" : "-- : --";

  Future<void> _openPicker() async {
    final result = await showDialog<Map<String, String>>(
      context: context,
      builder:
          (context) => _TimePickerDialog(
            hour: _hour,
            minute: _minute,
            period: _period,
          ),
    );
    if (result != null) {
      setState(() {
        _hour = result['hour']!;
        _minute = result['minute']!;
        _period = result['period']!;
        _value = "$_hour:$_minute $_period";
      });
      widget.onSelect?.call(_value);
    }
  }

  void _clearTime() {
    setState(() {
      _value = "";
      _hour = "";
      _minute = "";
      _period = "AM";
    });
    widget.onSelect?.call("");
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
            color: const Color(0xFF1C1C1E),
            borderRadius: BorderRadius.circular(8),
            border: Border.all(color: const Color(0xFF2C2C2E)),
          ),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.center,
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
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        const Icon(
                          Icons.access_time,
                          size: 16,
                          color: Colors.grey,
                        ),
                        const SizedBox(width: 6),
                        Text(
                          _displayTime,
                          style: const TextStyle(
                            fontSize: 14,
                            fontWeight: FontWeight.w500,
                            color: Colors.white,
                          ),
                          overflow: TextOverflow.ellipsis,
                        ),
                      ],
                    ),
                  ],
                ),
              ),
              Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  if (_value.isNotEmpty)
                    IconButton(
                      icon: const Icon(Icons.close, size: 16, color: Colors.grey),
                      onPressed: _clearTime,
                      constraints: const BoxConstraints(),
                      padding: EdgeInsets.zero,
                    ),
                  const SizedBox(width: 4),
                  const Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Icon(Icons.arrow_drop_up, size: 14, color: Colors.grey),
                      Icon(Icons.arrow_drop_down, size: 14, color: Colors.grey),
                    ],
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _TimePickerDialog extends StatefulWidget {
  final String hour;
  final String minute;
  final String period;

  const _TimePickerDialog({
    required this.hour,
    required this.minute,
    required this.period,
  });

  @override
  State<_TimePickerDialog> createState() => _TimePickerDialogState();
}

class _TimePickerDialogState extends State<_TimePickerDialog> {
  late String _selectedHour = widget.hour;
  late String _selectedMinute = widget.minute;
  late String _selectedPeriod = widget.period;

  final List<String> _hours = List.generate(
    12,
    (i) => (i + 1).toString().padLeft(2, '0'),
  );
  final List<String> _minutes = List.generate(
    12,
    (i) => (i * 5).toString().padLeft(2, '0'),
  );
  final List<String> _periods = const ['AM', 'PM'];

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      backgroundColor: const Color(0xFF1C1C1E),
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(16),
        side: const BorderSide(color: Color(0xFF2C2C2E)),
      ),
      content: SizedBox(
        width: 240,
        height: 240,
        child: Row(
          children: [
            _buildColumn(_hours, _selectedHour, (val) {
              setState(() => _selectedHour = val);
              _handleSelect();
            }),
            _buildDivider(),
            _buildColumn(_minutes, _selectedMinute, (val) {
              setState(() => _selectedMinute = val);
              _handleSelect();
            }),
            _buildDivider(),
            _buildColumn(_periods, _selectedPeriod, (val) {
              setState(() => _selectedPeriod = val);
              _handleSelect();
            }),
          ],
        ),
      ),
    );
  }

  void _handleSelect() {
    Navigator.of(context).pop({
      'hour': _selectedHour,
      'minute': _selectedMinute,
      'period': _selectedPeriod,
    });
  }

  Widget _buildDivider() {
    return Container(
      width: 1,
      height: double.infinity,
      color: const Color(0xFF2C2C2E),
      margin: const EdgeInsets.symmetric(horizontal: 4, vertical: 8),
    );
  }

  Widget _buildColumn(
    List<String> items,
    String selectedValue,
    ValueChanged<String> onSelected,
  ) {
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
                decoration: BoxDecoration(
                  color: isSelected ? Colors.blue : Colors.transparent,
                  borderRadius: BorderRadius.circular(8),
                ),
                child: Center(
                  child: Text(
                    item,
                    style: TextStyle(
                      color: isSelected ? Colors.white : Colors.grey,
                      fontWeight: isSelected ? FontWeight.bold : FontWeight.normal,
                      fontSize: 14,
                    ),
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
