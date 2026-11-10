import 'package:flutter/material.dart';
import '../data/types.dart';
import '../data/viewmodel.dart';
import 'flutter/time_picker_dialog.dart';

class InputTime extends StatefulWidget {
  final InputTimeProps props;
  final ValueChanged<String>? onSelect;

  const InputTime(this.props, {this.onSelect, super.key});

  @override
  State<InputTime> createState() => _InputTimeState();
}

class _InputTimeState extends State<InputTime> {
  late final InputTimeViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = InputTimeViewModel(widget.props, onSelect: widget.onSelect);
  }

  @override
  void dispose() {
    _vm.dispose();
    super.dispose();
  }

  Future<void> _openPicker() async {
    final result = await showDialog<Map<String, String>>(
      context: context,
      builder: (_) => TimePickerDialog(hour: _vm.hour, minute: _vm.minute, period: _vm.period),
    );
    _vm.applySelection(result);
  }

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: _vm,
      builder: (context, _) => SizedBox(
        width: double.infinity,
        child: InkWell(
          onTap: widget.props.inputProps.disabled ? null : _openPicker,
          borderRadius: BorderRadius.circular(8),
          child: Container(
            width: double.infinity,
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
            decoration: BoxDecoration(color: const Color(0xFF1C1C1E), borderRadius: BorderRadius.circular(8), border: Border.all(color: const Color(0xFF2C2C2E))),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.center,
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      if (widget.props.labelProps != null)
                        Text(widget.props.labelProps!.label.toUpperCase(), style: const TextStyle(fontSize: 10, color: Colors.grey, fontWeight: FontWeight.w600)),
                      const SizedBox(height: 2),
                      Row(mainAxisAlignment: MainAxisAlignment.center, children: [
                        const Icon(Icons.access_time, size: 16, color: Colors.grey),
                        const SizedBox(width: 6),
                        Text(_vm.displayTime, style: const TextStyle(fontSize: 14, fontWeight: FontWeight.w500, color: Colors.white), overflow: TextOverflow.ellipsis),
                      ]),
                    ],
                  ),
                ),
                if (_vm.value.isNotEmpty)
                  IconButton(icon: const Icon(Icons.close, size: 16, color: Colors.grey), onPressed: _vm.clearTime, constraints: const BoxConstraints(), padding: EdgeInsets.zero),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
