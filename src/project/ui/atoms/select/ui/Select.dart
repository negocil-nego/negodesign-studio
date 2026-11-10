import 'package:flutter/material.dart';
import '../data/types.dart';
import '../data/viewmodel.dart';
import 'flutter/select_dialog.dart';

class Select extends StatefulWidget {
  final SelectProps props;
  final ValueChanged<SelectOption>? onSelect;

  const Select(this.props, {this.onSelect, super.key});

  @override
  State<Select> createState() => _SelectState();
}

class _SelectState extends State<Select> {
  late final SelectViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = SelectViewModel(widget.props, onSelect: widget.onSelect);
  }

  @override
  void dispose() {
    _vm.dispose();
    super.dispose();
  }

  Future<void> _openSelectDialog() async {
    final result = await showDialog<SelectOption>(
      context: context,
      builder: (_) => SelectDialog(options: widget.props.options, selected: _vm.selected),
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
          onTap: widget.props.disabled ? null : _openSelectDialog,
          borderRadius: BorderRadius.circular(8),
          child: Container(
            width: double.infinity,
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
            decoration: BoxDecoration(
              color: Theme.of(context).cardColor,
              borderRadius: BorderRadius.circular(8),
              border: Border.all(color: Colors.blue.withValues(alpha: 0.5)),
            ),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      if (widget.props.label != null)
                        Text(widget.props.label!.toUpperCase(), style: const TextStyle(fontSize: 10, color: Colors.grey, fontWeight: FontWeight.w600)),
                      Text(_vm.displayLabel, style: const TextStyle(fontSize: 14, fontWeight: FontWeight.w500), overflow: TextOverflow.ellipsis),
                    ],
                  ),
                ),
                const Icon(Icons.keyboard_arrow_down, size: 18, color: Colors.grey),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
