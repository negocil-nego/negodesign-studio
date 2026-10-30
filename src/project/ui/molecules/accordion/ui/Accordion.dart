import 'package:flutter/material.dart';

class Accordion extends StatelessWidget {
  final String title;
  final String? count;
  final bool initiallyOpen;
  final Widget child;
  const Accordion({super.key, required this.title, this.count, this.initiallyOpen = false, required this.child});

  @override
  Widget build(BuildContext context) => Container(
    decoration: BoxDecoration(border: Border.all(color: Theme.of(context).dividerColor), borderRadius: BorderRadius.circular(12)),
    child: ExpansionTile(
      initiallyExpanded: initiallyOpen,
      title: Text(title),
      trailing: Row(mainAxisSize: MainAxisSize.min, children: [if (count != null) Text(count!), const SizedBox(width: 8), const Icon(Icons.expand_more)]),
      children: [Padding(padding: const EdgeInsets.all(16), child: child)],
    ),
  );
}
