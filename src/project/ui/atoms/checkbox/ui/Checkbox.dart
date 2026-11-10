import 'package:flutter/material.dart';
import '../../../../data/theme/dart/theme_data.dart';
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
    final primary = Theme.of(context).colorScheme.primary;
    return ListenableBuilder(
      listenable: _vm,
      builder: (context, _) => InkWell(
        onTap: widget.props.disabled ? null : () => _vm.toggle(!_vm.checked),
        borderRadius: BorderRadius.circular(NegoThemeData.radiusSm),
        child: Opacity(
          opacity: widget.props.disabled ? 0.5 : 1.0,
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              AnimatedContainer(
                duration: const Duration(milliseconds: 150),
                width: 16,
                height: 16,
                decoration: BoxDecoration(
                  color: _vm.checked ? primary : NegoThemeData.blenderSurface,
                  borderRadius: BorderRadius.circular(NegoThemeData.radiusSm),
                  border: Border.all(
                    color: _vm.checked ? primary : NegoThemeData.blenderBorder,
                  ),
                ),
                child: _vm.checked
                    ? const Icon(Icons.check, size: 12, color: Colors.white)
                    : null,
              ),
              if (widget.props.label != null) ...[
                const SizedBox(width: 10),
                Text(
                  widget.props.label!,
                  style: const TextStyle(fontSize: 13, color: NegoThemeData.textPrimary),
                ),
              ],
            ],
          ),
        ),
      ),
    );
  }
}
