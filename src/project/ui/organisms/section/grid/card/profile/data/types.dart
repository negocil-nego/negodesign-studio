import '../../../../../../molecules/card/profile/data/types.dart';
import '../../../data/types.dart';

class GridCardProfileProps {
  final SectionGridProps? sectionGridProps;
  final List<CardProfileProps> items;

  const GridCardProfileProps({
    this.sectionGridProps,
    required this.items,
  });
}
