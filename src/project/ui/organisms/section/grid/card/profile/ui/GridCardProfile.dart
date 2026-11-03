import 'package:flutter/widgets.dart';

import '../data/types.dart';
import '../../../../../../molecules/card/profile/ui/CardProfile.dart';
import '../../../ui/SectionGrid.dart';
import '../../../data/types.dart';

class GridCardProfile extends StatelessWidget {
  final GridCardProfileProps props;

  const GridCardProfile(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    final section = props.sectionGridProps;
    return SectionGrid(SectionGridProps(
      titleProps: section?.titleProps,
      descriptionProps: section?.descriptionProps,
      className: section?.className,
      children: props.items.map((item) => CardProfile(item)).toList(),
    ));
  }
}
