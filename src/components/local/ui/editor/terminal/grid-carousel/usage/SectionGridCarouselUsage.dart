import 'package:flutter/widgets.dart';
import '../../../../../../../project/ui/atoms/text/data/types.dart';
import '../../../../../../../project/ui/molecules/card/media/data/types.dart';
import '../../../../../../../project/ui/molecules/card/product/data/types.dart';
import '../../../../../../../project/ui/molecules/card/shared/card-title/data/types.dart';
import '../../../../../../../project/ui/molecules/card/shared/card-image/data/types.dart';
import '../../../../../../../project/ui/molecules/card/media/ui/CardMedia.dart';
import '../../../../../../../project/ui/molecules/card/product/ui/CardProduct.dart';
import '../../../../../../../project/ui/organisms/section/grid-carousel/data/types.dart';
import '../../../../../../../project/ui/organisms/section/grid-carousel/ui/SectionGridCarousel.dart';

class SectionGridCarouselExample extends StatelessWidget {
  const SectionGridCarouselExample({super.key});

  @override
  Widget build(BuildContext context) => SectionGridCarousel(
        SectionGridCarouselProps(
          titleProps: TextProps(Text: 'SectionGridCarousel'),
          descriptionProps: TextProps(Text: 'Organize cartões numa apresentação responsiva.'),
          rows: 2,
          columns: 2,
          children: [
            CardMedia(const CardMediaProps(
              titleProps: CardTitleProps(title: 'Design system'),
              imageProps: CardImageProps(src: '/img/ux-01.jpg', alt: 'Equipa a trabalhar'),
            )),
            CardProduct(const CardProductProps(
              titleProps: CardTitleProps(title: 'Componentes reutilizáveis'),
              imageProps: CardImageProps(src: '/img/programmer-01.jpg', alt: 'Código num monitor'),
            )),
          ],
        ),
      );
}
