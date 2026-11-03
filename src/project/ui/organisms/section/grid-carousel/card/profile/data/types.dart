import '../../../../../../molecules/card/profile/data/types.dart';
import '../../../data/types.dart';

class GridCarouselCardProfileProps {
  final SectionGridCarouselProps? sectionGridCarouselProps;
  final List<CardProfileProps> items;

  const GridCarouselCardProfileProps({
    this.sectionGridCarouselProps,
    required this.items,
  });
}
