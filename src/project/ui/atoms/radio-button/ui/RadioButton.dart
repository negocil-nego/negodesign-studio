import 'package:flutter/material.dart';
import '../../../../data/theme/dart/theme_data.dart';
import '../data/types.dart';
import '../data/radio_button_view_model.dart';

class RadioButton extends StatefulWidget {
  final RadioButtonProps props;
  const RadioButton(this.props, {super.key});

  @override
  State<RadioButton> createState() => _RadioButtonState();
}

class _RadioButtonState extends State<RadioButton> {
  late final RadioButtonViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = RadioButtonViewModel(widget.props);
  }

  @override
  void didUpdateWidget(covariant RadioButton oldWidget) {
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
      builder: (context, _) => Container(
        decoration: NegoThemeData.surfaceDecoration(),
        clipBehavior: Clip.antiAlias,
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: _vm.props.options.map((option) {
            final active = _vm.isSelected(option.value);
            return InkWell(
              onTap: (option.disabled || _vm.props.disabled) ? null : () => _vm.toggle(option.value),
              child: Container(
                padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 8),
                color: active ? primary : NegoThemeData.blenderSurface,
                child: Text(
                  option.label,
                  style: TextStyle(
                    fontSize: 12,
                    fontWeight: FontWeight.w500,
                    color: active ? Colors.white : NegoThemeData.textSecondary,
                  ),
                ),
              ),
            );
          }).toList(),
        ),
      ),
    );
  }
}
