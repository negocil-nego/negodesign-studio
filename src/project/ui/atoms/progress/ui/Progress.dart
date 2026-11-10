import 'package:flutter/material.dart';
import '../../../../data/theme/dart/theme_data.dart';
import '../data/types.dart';
import '../data/progress_view_model.dart';

class Progress extends StatefulWidget {
  final ProgressProps props;
  const Progress(this.props, {super.key});

  @override
  State<Progress> createState() => _ProgressState();
}

class _ProgressState extends State<Progress> {
  late final ProgressViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = ProgressViewModel(widget.props);
  }

  @override
  void didUpdateWidget(covariant Progress oldWidget) {
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
        height: 12,
        decoration: NegoThemeData.surfaceDecoration(radius: NegoThemeData.radiusLg),
        clipBehavior: Clip.antiAlias,
        child: Row(
          children: _vm.resolvedBars.map((bar) {
            final flex = bar.value.clamp(0, 100).round();
            return Expanded(
              flex: flex > 0 ? flex : 1,
              child: Container(color: _vm.resolveColor(bar.variant, primary)),
            );
          }).toList(),
        ),
      ),
    );
  }
}
