import 'package:flutter/material.dart';
import '../shared/card-image/ui/CardImage.dart';
import '../shared/card-thumbnail-video/data/types.dart';
import '../shared/card-thumbnail-video/ui/CardThumbnailVideo.dart';
import '../shared/card-video/ui/CardVideo.dart';
import 'types.dart';

class CardMediaViewModel {
  final CardMediaBaseProps props;
  const CardMediaViewModel(this.props);

  bool get hasHeaderDetails =>
      props.titleProps != null || props.ratingProps != null || props.priceProps != null;

  bool get hasActionButtons =>
      props.cartButtonProps != null || props.favoriteButtonProps != null;

  Widget buildMedia() {
    if (props.thumbnailVideoProps != null) {
      final t = props.thumbnailVideoProps!;
      return CardThumbnailVideo(
        CardThumbnailVideoProps(
          videoProps: t.videoProps,
          imageProps: t.imageProps,
          visibleButtonPlay: true,
        ),
      );
    }
    if (props.videoProps != null) return CardVideo(props.videoProps!);
    if (props.imageProps != null) return CardImage(props.imageProps!);
    return const SizedBox.shrink();
  }
}

