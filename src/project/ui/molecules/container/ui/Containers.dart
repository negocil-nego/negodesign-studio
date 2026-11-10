import 'package:flutter/material.dart';
import '../../../../data/theme/dart/theme_data.dart';
import '../data/types.dart';
import '../data/containers_view_model.dart';

class Containers extends StatefulWidget {
  final ContainersProps props;
  const Containers(this.props, {super.key});

  @override
  State<Containers> createState() => _ContainersState();
}

class _ContainersState extends State<Containers> {
  late final ContainersViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = ContainersViewModel(widget.props);
  }

  @override
  void didUpdateWidget(covariant Containers oldWidget) {
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
        padding: const EdgeInsets.all(24),
        decoration: NegoThemeData.cardDecoration(),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              _vm.props.title,
              style: const TextStyle(
                fontSize: 28,
                fontWeight: FontWeight.w600,
                color: NegoThemeData.textPrimary,
              ),
            ),
            const SizedBox(height: 10),
            Text(
              _vm.props.lead,
              style: const TextStyle(fontSize: 15, color: NegoThemeData.textSecondary),
            ),
            const Padding(
              padding: EdgeInsets.symmetric(vertical: 16),
              child: Divider(color: NegoThemeData.blenderBorder),
            ),
            Text(
              _vm.props.body,
              style: const TextStyle(fontSize: 13, color: NegoThemeData.textMuted),
            ),
            const SizedBox(height: 18),
            ElevatedButton(
              onPressed: _vm.triggerAction,
              style: ElevatedButton.styleFrom(backgroundColor: primary, foregroundColor: Colors.white),
              child: Text(_vm.props.actionLabel),
            ),
          ],
        ),
      ),
    );
  }
}
