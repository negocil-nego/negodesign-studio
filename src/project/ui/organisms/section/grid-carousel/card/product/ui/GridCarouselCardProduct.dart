import 'package:flutter/widgets.dart';

import '../data/types.dart';
import '../../../../../../molecules/card/product/ui/CardProduct.dart';
import '../../../ui/SectionGridCarousel.dart';
import '../../../data/types.dart';

class GridCarouselCardProduct extends StatelessWidget {
  final GridCarouselCardProductProps props;

  const GridCarouselCardProduct(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    final section = props.sectionGridCarouselProps;
    return SectionGridCarousel(SectionGridCarouselProps(
      titleProps: section?.titleProps,
      descriptionProps: section?.descriptionProps,
      className: section?.className,
      rows: section?.rows ?? 1,
      columns: section?.columns ?? 4,
      children: props.items.map((item) => CardProduct(item)).toList(),
    ));
  }
}
