import '../../../../../../molecules/card/media/data/types.dart';
import '../../../data/types.dart';

class GridCarouselCardMediaProps {
  final SectionGridCarouselProps? sectionGridCarouselProps;
  final List<CardMediaProps> items;

  const GridCarouselCardMediaProps({
    this.sectionGridCarouselProps,
    required this.items,
  });
}
