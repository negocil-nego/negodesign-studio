import 'package:flutter/widgets.dart';

import '../data/types.dart';
import '../../../../../../molecules/card/product/ui/CardProduct.dart';
import '../../../buttons/ui/SectionCarouselButtons.dart';
import '../../../buttons/data/types.dart';

class CarouselCardProduct extends StatelessWidget {
  final CarouselCardProductProps props;

  const CarouselCardProduct(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    final section = props.sectionCarouselButtonsProps;
    return SectionCarouselButtons(SectionCarouselButtonsProps(
      titleProps: section?.titleProps,
      descriptionProps: section?.descriptionProps,
      className: section?.className,
      children: props.items.map((item) => CardProduct(item)).toList(),
    ));
  }
}
