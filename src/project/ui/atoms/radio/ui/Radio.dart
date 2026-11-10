import 'package:flutter/material.dart';
import '../data/types.dart';
import '../data/viewmodel.dart';

class Radio extends StatefulWidget {
  final RadioProps props;
  const Radio(this.props, {super.key});

  @override
  State<Radio> createState() => _RadioState();
}

class _RadioState extends State<Radio> {
  late final RadioViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = RadioViewModel(widget.props);
  }

  @override
  void didUpdateWidget(covariant Radio oldWidget) {
    super.didUpdateWidget(oldWidget);
    _vm.didUpdate(widget.props);
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
      builder: (context, _) => Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: widget.props.options.map((option) {
          return RadioListTile<String>(
            title: Text(option.label),
            value: option.value,
            groupValue: _vm.selected,
            onChanged: (option.disabled || widget.props.disabled) ? null : _vm.select,
          );
        }).toList(),
      ),
    );
  }
}
