import 'package:flutter/material.dart';
import '../../../../../../data/theme/dart/theme_data.dart';
import '../data/types.dart';
import '../data/input_range_view_model.dart';

class InputRange extends StatefulWidget {
  final InputRangeProps props;
  const InputRange(this.props, {super.key});

  @override
  State<InputRange> createState() => _InputRangeState();
}

class _InputRangeState extends State<InputRange> {
  late final InputRangeViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = InputRangeViewModel(widget.props);
  }

  @override
  void didUpdateWidget(covariant InputRange oldWidget) {
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
    final primary = Theme.of(context).colorScheme.primary;
    return ListenableBuilder(
      listenable: _vm,
      builder: (context, _) => Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          if (_vm.props.label != null || _vm.props.showValue)
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                if (_vm.props.label != null)
                  Text(
                    _vm.props.label!,
                    style: const TextStyle(fontSize: 12, color: NegoThemeData.textSecondary),
                  ),
                if (_vm.props.showValue)
                  Text(
                    _vm.value.toStringAsFixed(0),
                    style: const TextStyle(fontSize: 11, color: NegoThemeData.textPrimary),
                  ),
              ],
            ),
          Slider(
            value: _vm.value,
            min: _vm.props.min,
            max: _vm.props.max,
            divisions: _vm.divisions,
            activeColor: primary,
            onChanged: _vm.props.disabled ? null : _vm.updateValue,
          ),
        ],
      ),
    );
  }
}
