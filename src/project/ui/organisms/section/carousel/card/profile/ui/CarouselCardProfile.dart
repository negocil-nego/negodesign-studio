import 'package:flutter/widgets.dart';

import '../data/types.dart';
import '../../../../../../molecules/card/profile/ui/CardProfile.dart';
import '../../../buttons/ui/SectionCarouselButtons.dart';
import '../../../buttons/data/types.dart';

class CarouselCardProfile extends StatelessWidget {
  final CarouselCardProfileProps props;

  const CarouselCardProfile(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    final section = props.sectionCarouselButtonsProps;
    return SectionCarouselButtons(SectionCarouselButtonsProps(
      titleProps: section?.titleProps,
      descriptionProps: section?.descriptionProps,
      className: section?.className,
      children: props.items.map((item) => CardProfile(item)).toList(),
    ));
  }
}
