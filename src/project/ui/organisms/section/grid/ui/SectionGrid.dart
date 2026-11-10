import 'package:flutter/material.dart';
import '../data/types.dart';
import '../data/viewmodel.dart';

class SectionGrid extends StatelessWidget {
  final SectionGridProps props;
  const SectionGrid(this.props, {super.key});

  static const _vm = SectionGridViewModel();

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 24),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          if (props.titleProps != null || props.descriptionProps != null)
            Padding(
              padding: const EdgeInsets.only(bottom: 24),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  if (props.titleProps != null)
                    Text(props.titleProps!.Text, style: const TextStyle(fontSize: 28, fontWeight: FontWeight.w600)),
                  if (props.descriptionProps != null)
                    Padding(padding: const EdgeInsets.only(top: 8), child: Text(props.descriptionProps!.Text)),
                ],
              ),
            ),
          LayoutBuilder(
            builder: (context, box) => GridView.count(
              shrinkWrap: true,
              physics: const NeverScrollableScrollPhysics(),
              crossAxisCount: _vm.columnsForWidth(box.maxWidth),
              mainAxisSpacing: 8,
              crossAxisSpacing: 8,
              childAspectRatio: 0.72,
              children: props.children
                  .map((child) => FittedBox(fit: BoxFit.scaleDown, alignment: Alignment.topCenter, child: child))
                  .toList(),
            ),
          ),
        ],
      ),
    );
  }
}
