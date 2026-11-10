import 'package:flutter/material.dart';
import '../data/types.dart';
import '../data/viewmodel.dart';

class Toggle extends StatefulWidget {
  final ToggleProps props;
  const Toggle(this.props, {super.key});

  @override
  State<Toggle> createState() => _ToggleState();
}

class _ToggleState extends State<Toggle> {
  late final ToggleViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = ToggleViewModel(widget.props);
  }

  @override
  void didUpdateWidget(covariant Toggle oldWidget) {
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
      builder: (context, _) => Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Switch(
            value: _vm.checked,
            onChanged: widget.props.disabled ? null : _vm.toggle,
          ),
          if (widget.props.label != null) ...[
            const SizedBox(width: 8),
            Text(widget.props.label!),
          ],
        ],
      ),
    );
  }
}
