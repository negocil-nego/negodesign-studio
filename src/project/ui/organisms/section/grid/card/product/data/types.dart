import '../../../../../../molecules/card/product/data/types.dart';
import '../../../data/types.dart';

class GridCardProductProps {
  final SectionGridProps? sectionGridProps;
  final List<CardProductProps> items;

  const GridCardProductProps({
    this.sectionGridProps,
    required this.items,
  });
}
