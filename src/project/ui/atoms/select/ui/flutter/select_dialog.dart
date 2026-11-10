import 'package:flutter/material.dart';
import '../../data/types.dart';
import '../../data/viewmodel.dart';

class SelectDialog extends StatefulWidget {
  final List<SelectOption> options;
  final SelectOption? selected;

  const SelectDialog({super.key, required this.options, this.selected});

  @override
  State<SelectDialog> createState() => _SelectDialogState();
}

class _SelectDialogState extends State<SelectDialog> {
  late final SelectViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = SelectViewModel(SelectProps(id: 'dialog', options: widget.options, value: widget.selected?.value));
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
        final filtered = _vm.filteredOptions;
        return AlertDialog(
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
          content: SizedBox(
            width: double.maxFinite,
            height: 380,
            child: Column(children: [
              TextField(
                decoration: const InputDecoration(hintText: 'Search...', isDense: true),
                onChanged: _vm.updateQuery,
              ),
              const SizedBox(height: 10),
              Expanded(
                child: ListView(
                  children: filtered.map((opt) {
                    final isSelected = widget.selected?.value == opt.value;
                    return ListTile(
                      leading: opt.icon != null ? Text(opt.icon!, style: const TextStyle(fontSize: 18)) : null,
                      title: Text(opt.label, style: TextStyle(color: isSelected ? Colors.blue : null)),
                      trailing: isSelected ? const Icon(Icons.check, color: Colors.blue, size: 18) : null,
                      onTap: () => Navigator.of(context).pop(opt),
                    );
                  }).toList(),
                ),
              ),
            ]),
          ),
        );
      },
    );
  }
}
