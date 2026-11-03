import '../../../../../../molecules/card/media/data/types.dart';
import '../../../data/types.dart';

class GridCardMediaProps {
  final SectionGridProps? sectionGridProps;
  final List<CardMediaProps> items;

  const GridCardMediaProps({
    this.sectionGridProps,
    required this.items,
  });
}
