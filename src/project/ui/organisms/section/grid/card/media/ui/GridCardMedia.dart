import 'package:flutter/widgets.dart';

import '../data/types.dart';
import '../../../../../../molecules/card/media/ui/CardMedia.dart';
import '../../../ui/SectionGrid.dart';
import '../../../data/types.dart';

class GridCardMedia extends StatelessWidget {
  final GridCardMediaProps props;

  const GridCardMedia(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    final section = props.sectionGridProps;
    return SectionGrid(SectionGridProps(
      titleProps: section?.titleProps,
      descriptionProps: section?.descriptionProps,
      className: section?.className,
      children: props.items.map((item) => CardMedia(item)).toList(),
    ));
  }
}
