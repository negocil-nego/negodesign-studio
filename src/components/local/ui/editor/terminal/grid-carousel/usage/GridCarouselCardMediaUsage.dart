import 'package:flutter/widgets.dart';
import '../../../../../../../project/ui/atoms/text/data/types.dart';
import '../../../../../../../project/ui/organisms/section/grid-carousel/data/types.dart';
import '../../../../../../../project/ui/molecules/card/media/data/types.dart';
import '../../../../../../../project/ui/molecules/card/shared/card-title/data/types.dart';
import '../../../../../../../project/ui/molecules/card/shared/card-image/data/types.dart';
import '../../../../../../../project/ui/organisms/section/grid-carousel/card/media/data/types.dart';
import '../../../../../../../project/ui/organisms/section/grid-carousel/card/media/ui/GridCarouselCardMedia.dart';

class GridCarouselCardMediaExample extends StatelessWidget {
  const GridCarouselCardMediaExample({super.key});

  @override
  Widget build(BuildContext context) => GridCarouselCardMedia(
        GridCarouselCardMediaProps(
          sectionGridCarouselProps: const SectionGridCarouselProps(
            titleProps: TextProps(Text: 'GridCarouselCardMedia'),
            descriptionProps: TextProps(Text: 'Cartões apresentados com um componente reutilizável.'),
            rows: 2,
            columns: 2,
          ),
          items: const [
            CardMediaProps(
              titleProps: CardTitleProps(title: 'Design system'),
              imageProps: CardImageProps(src: '/img/ux-01.jpg', alt: 'Equipa a trabalhar num design system'),
            ),
            CardMediaProps(
              titleProps: CardTitleProps(title: 'Componentes reutilizáveis'),
              imageProps: CardImageProps(src: '/img/programmer-01.jpg', alt: 'Componentes de software'),
            ),
          ],
        ),
      );
}
