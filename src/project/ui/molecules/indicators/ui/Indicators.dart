import 'package:flutter/material.dart';
import '../../../../data/theme/dart/theme_data.dart';
import '../data/types.dart';
import '../data/indicators_view_model.dart';

class Indicators extends StatefulWidget {
  final IndicatorsProps props;
  const Indicators(this.props, {super.key});

  @override
  State<Indicators> createState() => _IndicatorsState();
}

class _IndicatorsState extends State<Indicators> {
  late final IndicatorsViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = IndicatorsViewModel(widget.props);
  }

  @override
  void didUpdateWidget(covariant Indicators oldWidget) {
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
          ..._vm.visibleAlerts.map(
            (alert) => Container(
              margin: const EdgeInsets.only(bottom: 8),
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: _vm.variantColor(alert.variant, primary),
                borderRadius: BorderRadius.circular(NegoThemeData.radiusMd),
              ),
              child: Row(
                children: [
                  Expanded(
                    child: Text(alert.message, style: const TextStyle(color: Colors.white)),
                  ),
                  if (alert.dismissible)
                    IconButton(
                      icon: const Icon(Icons.close, size: 16, color: Colors.white),
                      onPressed: () => _vm.dismiss(alert.id),
                    ),
                ],
              ),
            ),
          ),
          Wrap(
            spacing: 6,
            runSpacing: 6,
            children: _vm.badges.map((b) {
              return Container(
                padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
                decoration: BoxDecoration(
                  color: _vm.variantColor(b.variant, primary),
                  borderRadius: BorderRadius.circular(b.pill ? 999 : NegoThemeData.radiusSm),
                ),
                child: Text(
                  b.label,
                  style: const TextStyle(fontSize: 11, color: Colors.white),
                ),
              );
            }).toList(),
          ),
        ],
      ),
    );
  }
}
