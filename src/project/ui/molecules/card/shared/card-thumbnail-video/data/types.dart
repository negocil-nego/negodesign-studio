import '../../card-image/data/types.dart';
import '../../card-video/data/types.dart';

class CardThumbnailVideoProps {
  final CardVideoProps videoProps;
  final CardImageProps imageProps;
  final bool visibleButtonPlay;

  const CardThumbnailVideoProps({
    required this.videoProps,
    required this.imageProps,
    this.visibleButtonPlay = false,
  });
}
