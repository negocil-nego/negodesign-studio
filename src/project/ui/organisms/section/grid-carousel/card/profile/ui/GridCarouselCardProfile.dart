import 'package:flutter/widgets.dart';

import '../data/types.dart';
import '../../../../../../molecules/card/profile/ui/CardProfile.dart';
import '../../../ui/SectionGridCarousel.dart';
import '../../../data/types.dart';

class GridCarouselCardProfile extends StatelessWidget {
  final GridCarouselCardProfileProps props;

  const GridCarouselCardProfile(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    final section = props.sectionGridCarouselProps;
    return SectionGridCarousel(SectionGridCarouselProps(
      titleProps: section?.titleProps,
      descriptionProps: section?.descriptionProps,
      className: section?.className,
      rows: section?.rows ?? 1,
      columns: section?.columns ?? 4,
      children: props.items.map((item) => CardProfile(item)).toList(),
    ));
  }
}
