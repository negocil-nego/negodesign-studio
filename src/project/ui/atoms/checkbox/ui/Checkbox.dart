import 'package:flutter/material.dart';
import '../data/types.dart';
import '../data/viewmodel.dart';

class Checkbox extends StatefulWidget {
  final CheckboxProps props;
  const Checkbox(this.props, {super.key});

  @override
  State<Checkbox> createState() => _CheckboxState();
}

class _CheckboxState extends State<Checkbox> {
  late final CheckboxViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = CheckboxViewModel(widget.props);
  }

  @override
  void didUpdateWidget(covariant Checkbox oldWidget) {
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
      builder: (context, _) => CheckboxListTile(
        value: _vm.checked,
        title: widget.props.label != null ? Text(widget.props.label!) : null,
        onChanged: widget.props.disabled ? null : _vm.toggle,
      ),
    );
  }
}
