import 'package:flutter/material.dart';
import '../../../../../../data/theme/dart/theme_data.dart';
import '../data/types.dart';
import '../data/block_button_view_model.dart';

class BlockButton extends StatefulWidget {
  final BlockButtonProps props;
  const BlockButton(this.props, {super.key});

  @override
  State<BlockButton> createState() => _BlockButtonState();
}

class _BlockButtonState extends State<BlockButton> {
  late final BlockButtonViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = BlockButtonViewModel(widget.props);
  }

  @override
  void didUpdateWidget(covariant BlockButton oldWidget) {
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
      builder: (context, _) => SizedBox(
        width: double.infinity,
        child: ElevatedButton(
          onPressed: _vm.props.disabled ? null : _vm.trigger,
          style: ElevatedButton.styleFrom(
            backgroundColor: _vm.isOutline
                ? Colors.transparent
                : (_vm.isSecondary ? NegoThemeData.blenderHover : primary),
            foregroundColor: Colors.white,
            padding: const EdgeInsets.symmetric(vertical: 14, horizontal: 20),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(NegoThemeData.radiusMd),
              side: BorderSide(color: _vm.isOutline ? primary : Colors.transparent),
            ),
          ),
          child: Text(_vm.props.label, style: const TextStyle(fontWeight: FontWeight.w600)),
        ),
      ),
    );
  }
}
