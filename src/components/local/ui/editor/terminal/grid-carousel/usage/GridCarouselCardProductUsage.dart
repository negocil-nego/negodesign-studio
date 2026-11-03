import 'package:flutter/widgets.dart';
import '../../../../../../../project/ui/atoms/text/data/types.dart';
import '../../../../../../../project/ui/organisms/section/grid-carousel/data/types.dart';
import '../../../../../../../project/ui/molecules/card/product/data/types.dart';
import '../../../../../../../project/ui/molecules/card/shared/card-title/data/types.dart';
import '../../../../../../../project/ui/molecules/card/shared/card-image/data/types.dart';
import '../../../../../../../project/ui/organisms/section/grid-carousel/card/product/data/types.dart';
import '../../../../../../../project/ui/organisms/section/grid-carousel/card/product/ui/GridCarouselCardProduct.dart';

class GridCarouselCardProductExample extends StatelessWidget {
  const GridCarouselCardProductExample({super.key});

  @override
  Widget build(BuildContext context) => GridCarouselCardProduct(
        GridCarouselCardProductProps(
          sectionGridCarouselProps: const SectionGridCarouselProps(
            titleProps: TextProps(Text: 'GridCarouselCardProduct'),
            descriptionProps: TextProps(Text: 'Cartões apresentados com um componente reutilizável.'),
            rows: 2,
            columns: 2,
          ),
          items: const [
            CardProductProps(
              titleProps: CardTitleProps(title: 'Design system'),
              imageProps: CardImageProps(src: '/img/ux-01.jpg', alt: 'Equipa a trabalhar num design system'),
            ),
            CardProductProps(
              titleProps: CardTitleProps(title: 'Componentes reutilizáveis'),
              imageProps: CardImageProps(src: '/img/programmer-01.jpg', alt: 'Componentes de software'),
            ),
          ],
        ),
      );
}
