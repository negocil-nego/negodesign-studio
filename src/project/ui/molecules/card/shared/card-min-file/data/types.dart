import '../../card-image/data/types.dart';

class CardMinFileProps {
  final CardImageProps? imageProps;
  final bool active;
  final void Function()? onClick;

  const CardMinFileProps({
    this.imageProps,
    this.active = false,
    this.onClick,
  });
}
