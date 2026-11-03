import 'package:flutter/material.dart';

import '../data/types.dart';

class SectionGrid extends StatelessWidget {
  final SectionGridProps props;

  const SectionGrid(this.props, {super.key});

  int _columns(double width) {
    if (width >= 2560) return 5;
    if (width >= 1536) return 4;
    if (width >= 768) return 3;
    return 1;
  }

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
                    Text(
                      props.titleProps!.Text,
                      style: const TextStyle(fontSize: 28, fontWeight: FontWeight.w600),
                    ),
                  if (props.descriptionProps != null)
                    Padding(
                      padding: const EdgeInsets.only(top: 8),
                      child: Text(props.descriptionProps!.Text),
                    ),
                ],
              ),
            ),
          LayoutBuilder(
            builder: (context, box) => GridView.count(
              shrinkWrap: true,
              physics: const NeverScrollableScrollPhysics(),
              crossAxisCount: _columns(box.maxWidth),
              mainAxisSpacing: 20,
              crossAxisSpacing: 20,
              childAspectRatio: 0.72,
              children: props.children,
            ),
          ),
        ],
      ),
    );
  }
}
