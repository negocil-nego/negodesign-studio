import 'package:flutter/material.dart';
import '../data/types.dart';
import '../data/viewmodel.dart';
import 'flutter/calendar_dialog.dart';

class InputDate extends StatefulWidget {
  final InputDateProps props;
  final ValueChanged<String>? onSelect;

  const InputDate(this.props, {this.onSelect, super.key});

  @override
  State<InputDate> createState() => _InputDateState();
}

class _InputDateState extends State<InputDate> {
  late final InputDateViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = InputDateViewModel(widget.props, onSelect: widget.onSelect);
  }

  @override
  void dispose() {
    _vm.dispose();
    super.dispose();
  }

  Future<void> _openPicker() async {
    final result = await showDialog<Map<String, int>>(
      context: context,
      builder: (_) => CalendarDialog(day: _vm.day, month: _vm.month, year: _vm.year, months: InputDateViewModel.months),
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
            decoration: BoxDecoration(
              color: Theme.of(context).cardColor,
              borderRadius: BorderRadius.circular(8),
              border: Border.all(color: Colors.orange.withValues(alpha: 0.5)),
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
                          style: const TextStyle(fontSize: 10, color: Colors.grey, fontWeight: FontWeight.w600),
                        ),
                      const SizedBox(height: 2),
                      Row(children: [
                        const Icon(Icons.calendar_today_outlined, size: 16, color: Colors.grey),
                        const SizedBox(width: 6),
                        Text(_vm.value, style: const TextStyle(fontSize: 14, fontWeight: FontWeight.w500), overflow: TextOverflow.ellipsis),
                      ]),
                    ],
                  ),
                ),
                const Icon(Icons.unfold_more, size: 18, color: Colors.grey),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
