import 'package:flutter/widgets.dart';

import '../data/types.dart';
import '../../../../../../molecules/card/media/ui/CardMedia.dart';
import '../../../buttons/ui/SectionCarouselButtons.dart';
import '../../../buttons/data/types.dart';

class CarouselCardMedia extends StatelessWidget {
  final CarouselCardMediaProps props;

  const CarouselCardMedia(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    final section = props.sectionCarouselButtonsProps;
    return SectionCarouselButtons(SectionCarouselButtonsProps(
      titleProps: section?.titleProps,
      descriptionProps: section?.descriptionProps,
      className: section?.className,
      children: props.items.map((item) => CardMedia(item)).toList(),
    ));
  }
}
