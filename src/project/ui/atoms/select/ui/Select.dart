import 'package:flutter/material.dart';
import '../data/types.dart';

class Select extends StatefulWidget {
  final SelectProps props;
  final ValueChanged<SelectOption>? onSelect;

  const Select(this.props, {this.onSelect, super.key});

  @override
  State<Select> createState() => _SelectState();
}

class _SelectState extends State<Select> {
  SelectOption? _selected;

  @override
  void initState() {
    super.initState();
    if (widget.props.value != null) {
      _selected = widget.props.options.firstWhere(
        (o) => o.value == widget.props.value || o.label == widget.props.value,
        orElse: () => widget.props.options.first,
      );
    }
  }

  Future<void> _openSelectDialog() async {
    final result = await showDialog<SelectOption>(
      context: context,
      builder:
          (context) =>
              _SelectDialog(options: widget.props.options, selected: _selected),
    );
    if (result != null) {
      setState(() => _selected = result);
      widget.onSelect?.call(result);
    }
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
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
            border: Border.all(color: Colors.blue.withOpacity(0.5)),
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
                      Text(
                        widget.props.label!.toUpperCase(),
                        style: const TextStyle(
                          fontSize: 10,
                          color: Colors.grey,
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                    Text(
                      _selected?.label ??
                          widget.props.placeholder ??
                          'Selecione...',
                      style: const TextStyle(
                        fontSize: 14,
                        fontWeight: FontWeight.w500,
                      ),
                      overflow: TextOverflow.ellipsis,
                    ),
                  ],
                ),
              ),
              const Icon(Icons.home, size: 18, color: Colors.grey),
            ],
          ),
        ),
      ),
    );
  }
}

class _SelectDialog extends StatefulWidget {
  final List<SelectOption> options;
  final SelectOption? selected;

  const _SelectDialog({required this.options, this.selected});

  @override
  State<_SelectDialog> createState() => _SelectDialogState();
}

class _SelectDialogState extends State<_SelectDialog> {
  String query = '';

  @override
  Widget build(BuildContext context) {
    final filtered =
        widget.options
            .where((o) => o.label.toLowerCase().contains(query.toLowerCase()))
            .toList();

    return AlertDialog(
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      content: SizedBox(
        width: double.maxFinite,
        height: 380,
        child: Column(
          children: [
            TextField(
              decoration: const InputDecoration(
                hintText: 'Search...',
                isDense: true,
              ),
              onChanged: (val) => setState(() => query = val),
            ),
            const SizedBox(height: 10),
            Expanded(
              child: ListView(
                children:
                    filtered.map((opt) {
                      final isSelected = widget.selected?.value == opt.value;
                      return ListTile(
                        leading:
                            opt.icon != null
                                ? Text(
                                  opt.icon!,
                                  style: const TextStyle(fontSize: 18),
                                )
                                : null,
                        title: Text(
                          opt.label,
                          style: TextStyle(
                            color: isSelected ? Colors.blue : null,
                          ),
                        ),
                        trailing:
                            isSelected
                                ? const Icon(
                                  Icons.check,
                                  color: Colors.blue,
                                  size: 18,
                                )
                                : null,
                        onTap: () => Navigator.of(context).pop(opt),
                      );
                    }).toList(),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
