import 'package:flutter/widgets.dart';
import '../../../../../../../project/ui/atoms/text/data/types.dart';
import '../../../../../../../project/ui/organisms/section/carousel/buttons/data/types.dart';
import '../../../../../../../project/ui/molecules/card/product/data/types.dart';
import '../../../../../../../project/ui/molecules/card/shared/card-title/data/types.dart';
import '../../../../../../../project/ui/molecules/card/shared/card-image/data/types.dart';
import '../../../../../../../project/ui/organisms/section/carousel/card/product/data/types.dart';
import '../../../../../../../project/ui/organisms/section/carousel/card/product/ui/CarouselCardProduct.dart';

class CarouselCardProductExample extends StatelessWidget {
  const CarouselCardProductExample({super.key});

  @override
  Widget build(BuildContext context) => CarouselCardProduct(
        CarouselCardProductProps(
          sectionCarouselButtonsProps: const SectionCarouselButtonsProps(
            titleProps: TextProps(Text: 'CarouselCardProduct'),
            descriptionProps: TextProps(Text: 'Cartões apresentados com um componente reutilizável.'),
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
