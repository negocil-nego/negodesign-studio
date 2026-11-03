import 'package:flutter/widgets.dart';
import '../../../../../../../project/ui/atoms/text/data/types.dart';
import '../../../../../../../project/ui/organisms/section/grid/data/types.dart';
import '../../../../../../../project/ui/molecules/card/media/data/types.dart';
import '../../../../../../../project/ui/molecules/card/shared/card-title/data/types.dart';
import '../../../../../../../project/ui/molecules/card/shared/card-image/data/types.dart';
import '../../../../../../../project/ui/organisms/section/grid/card/media/data/types.dart';
import '../../../../../../../project/ui/organisms/section/grid/card/media/ui/GridCardMedia.dart';

class GridCardMediaExample extends StatelessWidget {
  const GridCardMediaExample({super.key});

  @override
  Widget build(BuildContext context) => GridCardMedia(
        GridCardMediaProps(
          sectionGridProps: const SectionGridProps(
            titleProps: TextProps(Text: 'GridCardMedia'),
            descriptionProps: TextProps(Text: 'Cartões apresentados com um componente reutilizável.'),
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
