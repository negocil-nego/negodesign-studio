import 'package:flutter/widgets.dart';

import '../data/types.dart';
import '../../../../../../molecules/card/product/ui/CardProduct.dart';
import '../../../ui/SectionGrid.dart';
import '../../../data/types.dart';

class GridCardProduct extends StatelessWidget {
  final GridCardProductProps props;

  const GridCardProduct(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    final section = props.sectionGridProps;
    return SectionGrid(SectionGridProps(
      titleProps: section?.titleProps,
      descriptionProps: section?.descriptionProps,
      className: section?.className,
      children: props.items.map((item) => CardProduct(item)).toList(),
    ));
  }
}
